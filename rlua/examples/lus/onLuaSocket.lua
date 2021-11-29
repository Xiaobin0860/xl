local cc = cc.Director:getInstance():getScheduler()

---@class Net
local cls = class(...)

---[[typedef
---@alias NetMsg        number | "'onOpen'" | "'onClose'"
---@alias NetMsgHandler fun(type:NetMsg, msg:string):any | fun(self:Net, openedSocket:any):any
---typedef]]
;
---[[private static field
---@type table<NetMsg,boolean> @不打log的NetMsg的集合
local notLogged = {}
---private static field]]
;
---[[public static method
---@vararg NetMsg
---@return nil
function cls.logOff(...)
    for i = 1, math.huge do
        local netMsg = select(i, ...)
        if netMsg then
            notLogged[netMsg] = true
        else
            break
        end
    end
end

---@vararg NetMsg
---@return nil
function cls.logOn(...)
    for i = 1, math.huge do
        local netMsg = select(i, ...)
        if netMsg then
            notLogged[netMsg] = nil
        else
            break
        end
    end
end
---public static method]]
;
---[[private instance field
local weakRef = { __mode = 'k' }
---@type table<Net,any> @值为LuaSocket实例
local socketOf = setmetatable({}, weakRef)
---@type table<Net,any> @值为注册到cocos的lua函数的Id
local ccFnIdOf = setmetatable({}, weakRef)
---@type table<Net,string> @值为暂未接收完整的消息体
local bufferOf = setmetatable({}, weakRef)
---private instance field]]
;
---[[private instance method
---@param self Net
---@return nil
local function onConnected(self)
    local function get()
        local all, err, part = socketOf[self]:receive '*a'
        local data = bufferOf[self] .. (all or part)
        local len = #data
        while len >= 4 do
            all = luabpack.bunpack('i', data:sub(1, 4))
            if all < 0 then
                all = 0x100000000 + all
            end
            all = 4 + all
            if len < all then
                break
            end
            local msg, msgType, code = data:sub(5, all)
            msgType, msg, code = my.decPack(msg)
            if not notLogged[msgType] then
                printInfo('recv %d', msgType)
            end
            ---@type NetMsgHandler
            local onGet = self[msgType]
            self[msgType] = nil
            if code and code ~= 0 then
                printInfo('网关层错误: cmd=%d, code=%d', msgType, code)
                -- 就此打住, 等待触发onTimeout(默认重新连接)
            elseif onGet then
                onGet(msgType, msg)
            elseif Dispatcher then
                Dispatcher:dispatchEvent {
                    name = msgType;
                    msg = msg;
                }
            end
            data = data:sub(all + 1)
            len = len - all
        end
        bufferOf[self] = data
        if err == 'closed' then
            self:close()
        end
    end

    assert(not self.class[self.name])
    self.class[self.name] = self
    bufferOf[self] = ''
    socketOf[self]:settimeout(0) -- for get
    ccFnIdOf[self] = cc:scheduleScriptFunc(get, 0, false)

    if self.onOpen then
        self:onOpen(socketOf[self])
    end
end
---private instance method]]
;
---[[public instance method
function cls:ctor(name)
    ---[[public instance field
    self.name = assert(name)
    ---public instance field]]
end

---@param msgType NetMsg
---@param fn NetMsgHandler
---@return NetMsgHandler @旧的NetMsgHandler
function cls:setMsgHandler(msgType, fn)
    local old = self[msgType]
    self[msgType] = fn
    return old
end

---@return nil | string @nil on success; string describes failure.
function cls:open(host, port, timeout)
    timeout = timeout or 3
    if ccFnIdOf[self] then
        return 'opened'
    end
    local ok, err = socket.dns.getaddrinfo(host)
    if ok then
        if ok[1].family == 'inet6' then
            socketOf[self] = socket.tcp6()
        else
            socketOf[self] = socket.tcp()
        end
        socketOf[self]:settimeout(timeout)
        ok, err = socketOf[self]:connect(host, port)
        if ok then
            return onConnected(self)
        end
    end
    --[[luasocket usocket.c wsocket.c
        "address already in use"
        "already connected"
        "closed"
        "connection refused" 服务器没开
        "host or service not provided, or not known" 没网络, 切网时出现过这个错误
        "No address associated with hostname" 安卓没网络
        "permission denied"
        "timeout" 无法访达host:port
        "unknown error"
        ...
    --]]
    print(('Net:open(%s:%s) FAIL: %s'):format(host, port, err))
    self:close()
    return err
end

---@return nil
function cls:close()
    ---@type NetMsgHandler
    local usrFn
    if ccFnIdOf[self] then -- opened
        usrFn = self.onClose
        cc:unscheduleScriptEntry(ccFnIdOf[self])
        ccFnIdOf[self] = nil
    end
    if socketOf[self] then
        socketOf[self]:close()
        socketOf[self] = nil
    end
    bufferOf[self] = nil
    self.class[self.name] = nil

    if usrFn then
        for k in pairs(self) do
            if type(k) == 'number' then
                self[k] = nil
            end
        end
        usrFn(self)
    end
end

---for test
local test_last_packet = nil
function cls:test_resend()
    if not test_last_packet then
        return
    end
    socketOf[self]:send(test_last_packet)
end

---@param msgType NetMsg
---@param onGet NetMsgHandler | "function(cmd, s)end"
---@return nil | "duplicated request"
function cls:put(msgType, msg, onGet)
    if onGet then
        if self[msgType + 1] then
            return 'duplicated request'
        end
        self:setMsgHandler(msgType + 1, onGet)
    end
    local packet = pb.ImportAndNew('Packet', 'Packet')
    packet.c = msgType

    local log = not notLogged[msgType]
    if msg then
        if log then printInfo('send msg %d', msgType) end
        packet.s = msg:SerializeToString()
        if log then printInfo(msg) end
    else
        if log then printInfo('send cmd %d', msgType) end
    end
    do
        local chars = {}
        for i = 1, 32 do
            chars[i] = math.random(0, 254)
        end
        chars = string.char(unpack(chars))
        my.pack(packet, chars)
    end

    packet = my.encPack(packet)
    packet = luabpack.bpack('i', #packet) .. packet
    local len, err = socketOf[self]:send(packet)
    if err == 'closed' then
        self:close()
    end
    if DEBUG > 0 then
        test_last_packet = packet
        if not err and len > 1400 then
            local tag = ('!'):rep(36)
            printLog(tag, 'message=%d, length=%d', msgType, len)
        end
    end
end

function cls:onStatusChanged(s)
    -- Do nothing.
    -- Call setMsgHandler('onStatusChanged', ...) to do things.
end
---public instance method]]
;
return cls