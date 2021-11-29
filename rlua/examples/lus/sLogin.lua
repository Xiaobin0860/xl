local mvc = cc.load 'mvc'

local LoginScene = class(..., mvc.Scene(...))

function LoginScene:showWithScene(transition, time, more)
    self:show()
    display.runScene(self, transition, time, more)
    return self
end

function LoginScene:onEnter()
    self.root = self:findChild 'root'
    self.dbFctr = ccdb.CCFactory.new()
    self.pBg = self.root:getChildByName 'login_bg'
    if self.pBg:getChildByName("A") then
        self:setLoginAnimation()
    else
        local ratio = display.sizeInPixels.width / display.sizeInPixels.height
        if ratio > 16 / 9 then
            self.pBg:setScale(display.width / 1280)
        else
            self.pBg:setScale(display.height / CC_DESIGN_RESOLUTION.height)
        end
    end
end

function LoginScene:onEnterTransitionFinish()
    self.need_guide_battle = false

    self.pHotfix = self.root:getChildByName '1_hotfix'
    self.pLogin = self.root:getChildByName '2_login'
    self.btn_age = self.root:getChildByName("btn_age")
    local btn_login = self.pLogin:getChildByName("btn_login")
    btn_login:addClickEventWithSound(handler(self, self.login))
    self.btn_age:addClickEventWithSound(handler(self, self.age))
    local btn_sign = self.pLogin:getChildByName("btn_sign_up")
    local ok, tl = pcall(require, 'test.test_login')
    if ok then
        tl(btn_sign, self)
    end
    btn_sign:addClickEventListener(handler(self, self.signUp))

    self.pZone = self.root:getChildByName '3_zone'
    self.pZone:getChildByName('btn_choose')
    :addClickEventWithSound(handler(self, self.showZoneList))
    self.pZone:getChildByName('btn_start')
    :addClickEventWithSound(handler(self, self.enterZone))
    self.pCreate = self.root:getChildByName '4_create'
    self.rolepos = self.pCreate:getChildByName('pos')
    self.rolepos.posx = self.rolepos:getPositionX()
    self.pZone_argee = self.pZone:getChildByName("fangkuai")
    self.pZone_argee.selected = cc.UserDefault:getInstance():getBoolForKey("login_argee", false)
    self.pZone_argee:getChildByName("duihao"):setVisible(self.pZone_argee.selected)
    self.pZone_argee:addClickEventWithSound(function()
        self.pZone_argee.selected = not self.pZone_argee.selected
        cc.UserDefault:getInstance():setBoolForKey("login_argee", self.pZone_argee.selected)
        cc.UserDefault:getInstance():flush()
        self.pZone_argee:getChildByName("duihao"):setVisible(self.pZone_argee.selected)
    end)
    self.pZone:getChildByName("xy"):addClickEventWithSound(function()
        require('app.views.login.sLogin_agreement'):create():addToCenter(self)
    end)
    if SHOW_AGREEMENT then
        self.pZone_argee:show()
        self.pZone:getChildByName("label"):show()
        self.pZone:getChildByName("xy"):show()
    else
        self.pZone_argee:hide()
        self.pZone:getChildByName("label"):hide()
        self.pZone:getChildByName("xy"):hide()
    end

    self.fixBtn = self.root:getChildByName('btns'):getChildByName('fix'):hide()
    self.fixBtn:addClickEventWithSound(handler(self, self.fix))
    self.annoBtn = self.root:getChildByName('btns'):getChildByName('anno'):hide()
    self.annoBtn:addClickEventWithSound(handler(self, self.openAnnouncement))
    display.dockBottom(self.root:getChildByName 'health')
    display.dockBottom(self.root:getChildByName 'version')
    display.dockLeft(self.root:getChildByName('btns'))
    display.dockLeft(self.btn_age)
    display.dockBottom(self.btn_age)
    display.dockBottom(self.pHotfix)

    self:refreshVersionUI()
    require('app.models.login').initSDK(self)

    local function urlEncode(s)
        s = string.gsub(s, "([^%w%.%- ])", function(c) return string.format("%%%02X", string.byte(c)) end)
        return string.gsub(s, " ", "+")
    end

    local Bridge = require 'cocos.cocos2d.luaj' or require 'cocos.cocos2d.luaoc'
    if Bridge then
        local ok, info = Bridge.callStaticMethod('SDK', 'getPlatformInfo', { '', PACK_ID }, '(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;')
        if ok then
            cc.load('utils').http(
            'm=init_info&a=ad_info',
            {
                data = my.base64Encode(info)
            },
            function(res)
            end)
        end
    end

    --公告修复开关
    require 'cocos.cocos2d.json'
    cc.load('utils').http('m=init_info&a=button_switch', {
        channel_package_id = PACK_ID;
        build = BUILD;
        resource = RESOURCE;
    }, function(res)
        local ok, err = pcall(json.decode, res)
        if not ok then
            print(res)
            return
        end
        res = err
        if self.fixBtn then
            self.fixBtn:setVisible(res.button_switch_repair == '1')
            self.annoBtn:setVisible(res.button_switch_bulletin == '1')
        end
    end)

end

function LoginScene:onExit()
    local acc_layer = self.pBg:getChildByName("acc_layer")
    if acc_layer then
        acc_layer:removeAccelerate()
    end
    require('app.models.login').sceneExit()
end

function LoginScene:age()
    cc.load("widget").TipRules:create(my.fmt "适龄提示", my.fmt "1） 本游戏是一款2D挂机卡牌手机游戏，适用于年满16周岁及以上的用户，建议未成年人在家长的监护下使用游戏产品。\n" ..
    "2） 本游戏基于架空的故事背景。讲述了在神州世界中，神秘人率领其爪牙为祸四方，封印了神州大陆的所有侍从。玩家将扮演救世主，需要将被封印的侍从唤醒，才能与神秘人抗争并赢得战斗，拯救神州世界。\n" ..
    "3） 本游戏中有用户实名认证系统，不会以任何形式向认证为未成年和未认证的用户提供游戏服务，认证为未成年和未认证的用户无法进入游戏及充值。")
end

function LoginScene:onCleanup()
    package.loaded['app.views.sLogin'] = nil
end

function LoginScene:setLoginAnimation()
    self.pBg:getChildByName("xuanguang_ske"):getAnimation():getState("newAnimation").timeScale = 0.8
    local P_A = self.pBg:getChildByName("A")
    local P_B = self.pBg:getChildByName("B")
    local role1 = P_A:getChildByName("nanzhu_ske")
    local role2 = P_A:getChildByName("nanzhu_ske")
    local anim_state1 = role1:getAnimation():gotoAndPlayByProgress("newAnimation", 0.5, 0)
    anim_state1.timeScale = 0.8
    local role2 = P_A:getChildByName("jiuwei_ske")
    role2:getAnimation():getState("newAnimation").timeScale = 0.8
    local acc_layer = self.pBg:getChildByName("acc_layer")
    if not acc_layer then
        acc_layer = display.newLayer():addTo(self.pBg)
        local pa0 = P_A:getPositionY()
        local pb0 = P_B:getPositionY()

        local t, tick = 0, 0
        local first = true
        acc_layer:onAccelerate(function(x, y, z, timestamp)
            local delta = timestamp - t
            t = timestamp
            tick = tick + delta
            if tick > 0.5 then
                tick = 0
                y = cc.clampf(y, -1, 0)
                if first then  --第一次直接设置位置,防止晃动
                    P_A:setPositionY(pa0 + 60 * y)
                    P_B:setPositionY(pb0 + 60 * y * 0.5)
                    first = false
                else
                    P_A:moveTo({ y = pa0 + 60 * y, time = 0.5 })
                    P_B:moveTo({ y = pb0 + 60 * y * 0.5, time = 0.5 })
                end
            end
        end)
    end
    self.pBg:getChildByName("btn_login"):addClickEventListener(function()
        require('app.models.login').loginSDK()
    end)
end

function LoginScene:askOpenUrl(deque)
    self.hfQue = deque or self.hfQue
    local front = self.hfQue:front()
    local ask = require('app.views.login.sLogin_update'):create()
    ask._root:getChildByName('lbl_main')
    :setString(my.fmt('发现新版本v{0}，大小约{1}MB。是否跳转下载链接？',
    front.build, string.format("%.1f", math.max(front.size / 1000, 0.1))
    ))
    ask._root:getChildByName('btn_yes')
    :addClickEventWithSound(function()
        cc.Application:getInstance():openURL(front.url)
    end)
    :getTitleLabel():setString(my.fmt '跳转')
    ask._root:getChildByName('btn_no')
    :addClickEventWithSound(function()
        if front.full_force == 'force' then
            if device.platform == "ios" then
                os.exit()
            else
                cc.Director:getInstance():endToLua()
            end
        else
            ask:removeFromParent()
            self.hfQue:pop()
            if self.hfQue:size() == 0 then
                return require('app.models.login').loginSDK()
            end
            front = self.hfQue:front()
            if front.islenggeng == '1' then
                self:askOpenUrl()
            else
                self:askHotfix(self.hfQue)
            end
        end
    end)
    :getTitleLabel():setString(my.fmt(front.full_force == 'force' and '结束' or '跳过'))
    ask:addToCenter(self)
end

function LoginScene:askHotfix(deque)
    if deque then
        self.hfQue = deque
        self.hfNum = deque:size()
    else
        deque = self.hfQue
    end
    local MB = 0
    for _, v in deque:iter() do
        MB = MB + v.size / 1000
    end
    self.hfSize = string.format("%.1f", math.max(MB, 0.1))
    local ask = my.netTech and my.netTech():lower() ~= 'wifi'
    local function nextStep()
        if ask then ask:removeSelf() end
        self:showHotfixUI()
        self.fixBtn:hide()
        require('app.models.login').hotfix()
    end
    if ask then
        ask = require('app.views.login.sLogin_update'):create()
        ask._root:getChildByName('lbl_main')
        :setString(my.fmt('本次更新的文件大小约为{0}MB',
        self.hfSize
        ))
        ask._root:getChildByName('btn_yes')
        :addClickEventWithSound(nextStep)
        ask._root:getChildByName('btn_no')
        :addClickEventWithSound(function()
            if device.platform == "ios" then
                os.exit()
            else
                cc.Director:getInstance():endToLua()
            end
        end)
        ask:addToCenter(self)
    else
        nextStep()
    end
end

function LoginScene:askRetry(what)
    local ask = require('app.views.login.sLogin_ask'):create()
    if what == 'query_update' then
        ask._root:getChildByName('lbl_title'):show()
        ask._root:getChildByName('lbl_wifi'):show()
        ask._root:getChildByName('lbl_main')
        :setString(my.fmt '检查更新失败，是否重试？')
        ask._root:getChildByName('btn_yes')
        :addClickEventWithSound(function()
            ask:removeSelf()
            require('app.models.login').queryUpdate()
        end)
        :getTitleLabel():setString(my.fmt '重试')

        ask._root:getChildByName('btn_no')
        :addClickEventWithSound(function()
            if device.platform == "ios" then
                os.exit()
            else
                cc.Director:getInstance():endToLua()
            end
        end)
        :getTitleLabel():setString(my.fmt '结束')
    elseif what == 'hotfix' then
        ask._root:getChildByName('lbl_title'):show()
        ask._root:getChildByName('lbl_wifi'):show()
        ask._root:getChildByName('lbl_main')
        :setString(my.fmt('更新包{0}下载失败，是否重试？',
        self.hfNum - self.hfQue:size() + 1
        ))
        ask._root:getChildByName('btn_yes')
        :addClickEventWithSound(function()
            ask:removeSelf()
            require('app.models.login').hotfix()
        end)
        :getTitleLabel():setString(my.fmt '重试')

        ask._root:getChildByName('btn_no')
        :addClickEventWithSound(function()
            if device.platform == "ios" then
                os.exit()
            else
                cc.Director:getInstance():endToLua()
            end
        end)
        :getTitleLabel():setString(my.fmt '结束')
    elseif what == 'login' then
        ask._root:getChildByName('lbl_title'):hide()
        ask._root:getChildByName('lbl_wifi'):hide()
        ask._root:getChildByName('lbl_main')
        :setString(my.fmt '登录验证失败，是否重试？')
        ask._root:getChildByName('btn_yes')
        :addClickEventWithSound(function()
            ask:removeSelf()
            require('app.models.login').rebuildLoginScene()
        end)
        :getTitleLabel():setString(my.fmt '重试')

        ask._root:getChildByName('btn_no')
        :addClickEventWithSound(function()
            if device.platform == "ios" then
                os.exit()
            else
                cc.Director:getInstance():endToLua()
            end
        end)
        :getTitleLabel():setString(my.fmt '结束')
    end
    ask:addToCenter(self)
end

function LoginScene:hotfixProgress(pro, all)
    local ch = self.pHotfix:findChild 'progress'
    ch:setPercent(100 * pro / all)
    ch = self.pHotfix:getChildByName 'desc'
    ch:setString(my.fmt('更新包{0}/{1}，{2}/{3}KB，共{4}MB',
    self.hfNum - self.hfQue:size() + 1,
    self.hfNum,
    math.ceil(pro / 1000),
    math.ceil(all / 1000),
    self.hfSize
    ))
end

function LoginScene:hotfixDone()
    self.fixBtn:show()
    self:refreshVersionUI()
    if self.hfQue:size() > 0 then
        local front = self.hfQue:front()
        self:hotfixProgress(0, front.size * 1000)
    end
end

function LoginScene:refreshVersionUI()
    self.root:getChildByName('version')
    :setString(my.fmt('版本号{0}.{1}',
    table.concat(VERSION, '.'),
    RESOURCE
    ))
end

function LoginScene:showHotfixUI(isVisible)
    if isVisible == false then
        self.pHotfix:hide()
    else
        self.pHotfix:show()
        local ch = self.pHotfix:getChildByName 'bg'
        local size = ch:getContentSize()
        size.width = display.width
        ch:setContentSize(size)

        ch = ch:getChildByName 'progress'
        size = ch:getContentSize()
        size.width = display.width
        ch:setContentSize(size)
    end
end

function LoginScene:showPolicyTips(isVisible)
    self.root:getChildByName('health')
    :setVisible(isVisible ~= false)
end

function LoginScene:showLoginPanel()

    self.pLogin:show()
    local input_account = self.pLogin:getChildByName("input_account")
    input_account:getChildByName("PLACEHOLDER_LABEL"):hide()
    local font_path = require('app.models.ResManager').getFontPath()
    input_account:setFontName(font_path)
    input_account:setPlaceholderFontName(font_path)
    input_account:setText(cc.UserDefault:getInstance():getStringForKey 'id')

    local input_password = self.pLogin:getChildByName("input_password")
    input_password:getChildByName("PLACEHOLDER_LABEL"):hide()
    input_password:setFontName(font_path)
    input_password:setText(cc.UserDefault:getInstance():getStringForKey 'tk')
end

function LoginScene:showLoginBtn(isVisible)
    self.pBg:getChildByName("btn_login"):setVisible(isVisible ~= false)
    self.btn_age:setVisible(isVisible ~= false)
end

function LoginScene:login()
    local account = self.pLogin:getChildByName("input_account"):getText()
    if account == '' then
        cc.load("widget").PopWord:create(my.fmt '请输入账号')
        return
    end
    local password = self.pLogin:getChildByName("input_password"):getText()
    require('app.models.login').login(account, password)
end

function LoginScene:loginFailure(reason)
    local k, tip = pb:require 'Common'
    if reason == k.LOGIN_NO_ACCOUNT then
        tip = my.fmt '账号不存在'
    elseif reason == k.LOGIN_WRONG_PASS then
        tip = my.fmt '密码错误'
    elseif reason == k.LOGIN_REPEATED_ACCOUNT then
        tip = my.fmt '账号已存在'
    elseif reason == k.ANTI_ONLINE then
        tip = my.fmt '22:00~8:00禁止游戏'
    elseif reason == k.ANTI_ONLINE1 then
        tip = my.fmt '已满1.5小时禁止游戏'
    elseif reason == k.ANTI_ONLINE2 then
        tip = my.fmt '已满3小时禁止游戏'
    end
    local W = cc.load 'widget'
    if tip then
        W.TipBox:create(tip)
        :setConfirmBtn(require('app.views.setting.wSetting').toLogin)
    else
        W.PopWord:create(reason)
    end
    self:removeChildByName 'signUp'
end

function LoginScene:signUp()
    require('app.views.login.sLogin_signUp')
    :create():setName('signUp')
    :addTo(self)
end

function LoginScene:setupZone(Zone)
    if not Zone then return end
    self.curZone = Zone

    local k = pb:require 'ClientLogin'
    local ch = self.pZone:findChild 'flag'
    ch:setHighlighted(false)
    if Zone.is_new then
        ch:show():setEnabled(true):setHighlighted(true)
    elseif Zone.status == k.LoginRes_Zone_HOT then
        ch:show():setEnabled(true):setHighlighted(false)
    else
        ch:hide()
    end
    ch:setTouchEnabled(false)

    self.pZone:findChild('name')
    :setString(my.fmt('{0}({1}区)',
    Zone.name, Zone.zid
    ))
end

function LoginScene:showLastZone(LoginRes)
    self.hfQue = nil
    self.hfNum = nil
    self.hfSize = nil
    self.loginRes = LoginRes

    self:showLoginBtn(false)

    local signUp = self:getChildByName 'signUp'
    if signUp then
        if signUp._root:toAlbum() then
            cc.load('widget').PopWord
            :create(my.fmt '已存入相册')
        end
        signUp:removeSelf()
    end

    if not self.curZone then
        local zones = LoginRes.zones
        if LoginRes.last_id and LoginRes.last_id > 0 then
            for i = 1, #zones do
                local zone = zones[i]
                if zone.id == LoginRes.last_id then
                    self:setupZone(zone)
                    break
                end
            end
        elseif LoginRes.last_server_id and LoginRes.last_server_id > 0 then
            for i = 1, #zones do
                local zone = zones[i]
                if zone.zid == LoginRes.last_server_id then
                    self:setupZone(zone)
                    break
                end
            end
        end
        if not self.curZone then
            self:setupZone(zones[#zones])
        end
    end
    self.pLogin:hide()
    self.pZone:show()
    self.btn_age:show()
    local logo = self.root:getChildByName('logo'):show()
    local cX = (display.width / 1280 - 1) / 2
    logo:setPositionX(logo:getPositionX() * (1 + cX))

    if #LoginRes.users.info > 0 then
        require('app.models.user').client_id = LoginRes.users.info[1].client_id
    end
    local AudioManager = require("app.models.AudioManager")
    AudioManager.playmusic(Audio.main)
end

function LoginScene:enterZone()
    if self.pZone_argee:isVisible() and not self.pZone_argee.selected then
        cc.load("widget").PopWord:create(my.fmt("进入游戏前，请先勾选同意<font color='#FAB687'>服务协议</font>"))
    else
        local k = pb:require 'ClientLogin'
        -- if self.curZone.status == k.LoginRes_Zone_MAINTAINING then
        --     self:showZoneList()
        -- else
        cc.load("utils").profile("登陆", "点击开始游戏")
        require('app.models.login')
        .selectZone(self.curZone.zid, self.curZone.id)
        cc.exports.g_zname = self.curZone.name
        cc.exports.g_rid = self.curZone.rid  --大区id
        -- end
    end
end

function LoginScene:showZoneList()
    if self.pZone_argee:isVisible() and not self.pZone_argee.selected then
        cc.load("widget").PopWord:create(my.fmt("进入游戏前，请先勾选同意<font color='#FAB687'>服务协议</font>"))
    else
        require('app.views.login.sLogin_zones')
        :create(self.loginRes)
        :addToCenter(self, 0)
        :setName('zones')
    end
end

function LoginScene:showCreatePanel()
    local story = require('app.views.wStory'):create('p1', function()
        require("app.models.AudioManager").stop()
        --技能
        local ConfigManager = require("app.models.ConfigManager")
        local natureSkill = ConfigManager.loadConfig('nature_skill')
        local actSKills = {}
        local talent = ConfigManager.loadConfig('talent', 'id', function(item)
            if item.target == 'rep_act_skill' then
                table.insert(actSKills, item.nature_skill_id)
            end
        end)

        local pc = pb:require 'Common'
        self:removeChildByName 'zones'
        self.pZone:hide()
        self.btn_age:hide()
        self.root:getChildByName('btns'):hide()
        self.pCreate:show()
        self.pCreate:getChildByName('pos'):setLocalZOrder(20)
        self.pCreate:getChildByName('front_anim'):setLocalZOrder(21)
        self.pCreate:getChildByName('bg'):setContentSize(cc.size(display.width, display.height))
        self.pCreate:getChildByName('topleft'):setPosition(cc.p(-display.width / 2 + 60, display.height / 2))
        self.pCreate:getChildByName('juesedi_male'):show():setPositionX(-display.width / 2)
        self.pCreate:getChildByName('juesedi_female'):hide():setPositionX(-display.width / 2)
        local input = self.pCreate:findChild('big_panel'):getChildByName("input_nick")
        input:getChildByName("PLACEHOLDER_LABEL"):hide()
        input:limitLength(6)
        local head_boy = self.pCreate:findChild("head_boy")
        local Common = require("app.models.Common")
        local ResManager = require("app.models.ResManager")
        Common.setTextureScale(head_boy:getChildByName("head"), ResManager.getHeroHeadIconPath(101, pc.MALE))

        local head_girl = self.pCreate:findChild("head_girl")
        Common.setTextureScale(head_girl:getChildByName("head"), ResManager.getHeroHeadIconPath(101, pc.FEMALE))
        local gender = pc.MALE

        local function setRole(g)
            --侍从动画
            if self._anim then
                self._anim:removeFromParent()
            end
            local hero_id = 101
            self._anim = ResManager.createHeroArmature(hero_id, nil, g, self.dbFctr)
            if self._anim then
                local state = self._anim:getAnimation():play("stand", 0)
                self._anim:addDBEventListener(ccdb.EventObject.COMPLETE, function(event)
                    local anim_name = event.animationState:getName()
                    if anim_name ~= "stand" then
                        self.rolepos:runAction(cc.Spawn:create(cc.ScaleTo:create(0.2, 1, 1, 1), cc.MoveTo:create(0.2, cc.p(self.rolepos.posx, self.rolepos:getPositionY()))))
                        self._anim:getAnimation():fadeIn('stand', 0.1, 0)
                        performWithDelay(self, function()
                            self.playing = false
                        end, 0.5)

                        local allChilds = self.pCreate:getChildren()
                        for m = 1, #allChilds do
                            local child = allChilds[m]
                            if child:getName() ~= 'pos' and child:getName() ~= 'bg' then
                                child:runAction(cc.FadeIn:create(0.15))
                            end
                        end
                        self.pCreate:getChildByName('topleft'):runAction(cc.FadeIn:create(0.15))
                    end
                end)
            end
            local cfgScale = ENUM.HERO_SCALE[hero_id] or 1
            self._anim:setScale(cfgScale)
            self.pCreate:getChildByName('pos'):addChild(self._anim)
        end

        setRole(gender)

        local SKILL_CHANGE = {
            [3] = 10320,
            [4] = 12920,
            [5] = 3020,
            [6] = 13120
        }
        local function playSkill(sender)
            local skillId = sender.skillId
            local idx = sender.idx
            if not self.playing and self._anim then
                self.playing = true

                local shout = skillId .. '.mp3'
                if gender == pc.FEMALE then
                    shout = 'woman_' .. skillId .. '.mp3'
                elseif gender == pc.MALE then
                    shout = 'man_' .. skillId .. '.mp3'
                end
                if shout then
                    require('app.models.AudioManager').playsound(shout)
                end

                local sound = natureSkill[skillId].sound
                if sound then
                    require('app.models.AudioManager').playsound(sound)
                end

                --移动放缩
                local action = cc.Sequence:create(cc.Spawn:create(cc.ScaleTo:create(0.2, 0.9, 0.9, 1), cc.MoveTo:create(0.2, cc.p(0, self.rolepos:getPositionY()))), cc.CallFunc:create(function()
                    self._anim:getAnimation():fadeIn(skillId, 0, 1)
                    if idx > 2 then
                        local W = cc.load 'widget'
                        --受击动画
                        local effect = W.DragonBonesNode:create('animation/skill_' .. skillId, self.dbFctr)
                        if effect then
                            local rolePos = self.pCreate:getChildByName('pos')
                            effect:addTo(rolePos, 70)
                            :play('newAnimation')
                            :setComplete(function()
                                effect:removeFromParent()
                            end)
                        end
                    end
                end))

                self.rolepos:runAction(action)

                local allChilds = self.pCreate:getChildren()
                for m = 1, #allChilds do
                    local child = allChilds[m]
                    if child:getName() ~= 'pos' and child:getName() ~= 'bg' then
                        child:runAction(cc.FadeOut:create(0.2))
                    end
                end
                self.pCreate:getChildByName('topleft'):runAction(cc.FadeOut:create(0.2))

                if idx > 2 then
                    local W = cc.load 'widget'
                    -- --受击动画
                    -- local effect = W.DragonBonesNode:create('animation/skill_' .. skillId, self.dbFctr)
                    -- if effect then
                    --     local rolePos = self.pCreate:getChildByName('pos')
                    --     effect:addTo(rolePos, 70)
                    --     :play('newAnimation')
                    --     :setComplete(function()
                    --         effect:removeFromParent()
                    --     end)
                    -- end
                    --大背景动画
                    if SKILL_CHANGE[idx] then
                        local SK = W.DragonBonesNode:create('animation/SK_' .. SKILL_CHANGE[idx], self.dbFctr)
                        if SK then
                            SK:addToCenter(self.pCreate, 10)
                            :play('newAnimation')
                            :setComplete(function()
                                SK:removeFromParent()
                            end)
                        end
                    end
                end
            end
        end

        local skill = self.pCreate:getChildByName('skill')
        for i = 1, 6 do
            local sk = skill:getChildByName('sk' .. i):hide()
            sk.idx = i
            local skillId = actSKills[i]
            if skillId then
                sk.skillId = skillId
                Common.setTextureScale(sk:getRendererNormal(), ResManager.getSkillIconPath(skillId))
                local info = natureSkill[skillId]
                if info then
                    sk:show()
                    sk:getChildByName('node'):getChildByName('label'):setString(info.name)
                    sk:addClickEventWithSound(playSkill)
                end
            end
        end

        --头像
        head_boy:getChildByName("choose"):show()
        head_girl:getChildByName("icon"):setColor(cc.c3b(170, 170, 170))
        head_girl:getChildByName("icon"):getChildByName("label"):setColor(cc.c3b(170, 170, 170))
        head_girl:getChildByName("choose"):hide()
        head_boy:addClickEventWithSound(function(s)
            if not self.playing then
                head_boy:getChildByName("choose"):show()
                head_boy:getChildByName("icon"):setColor(cc.c3b(255, 255, 255))
                head_boy:getChildByName("icon"):getChildByName("label"):setColor(cc.c3b(255, 255, 255))
                head_girl:getChildByName("choose"):hide()
                head_girl:getChildByName("icon"):setColor(cc.c3b(170, 170, 170))
                head_girl:getChildByName("icon"):getChildByName("label"):setColor(cc.c3b(170, 170, 170))
                self.pCreate:getChildByName('juesedi_male'):show()
                self.pCreate:getChildByName('juesedi_female'):hide()
                gender = pc.MALE
                setRole(gender)
                playSkill(skill:getChildByName('sk1'))
            end
        end)
        head_girl:addClickEventWithSound(function(s)
            if not self.playing then
                head_boy:getChildByName("choose"):hide()
                head_boy:getChildByName("icon"):setColor(cc.c3b(170, 170, 170))
                head_boy:getChildByName("icon"):getChildByName("label"):setColor(cc.c3b(170, 170, 170))
                head_girl:getChildByName("choose"):show()
                head_girl:getChildByName("icon"):setColor(cc.c3b(255, 255, 255))
                head_girl:getChildByName("icon"):getChildByName("label"):setColor(cc.c3b(255, 255, 255))
                self.pCreate:getChildByName('juesedi_female'):show()
                self.pCreate:getChildByName('juesedi_male'):hide()
                gender = pc.FEMALE
                setRole(gender)
                playSkill(skill:getChildByName('sk1'))
            end
        end)
        local btn_create = self.pCreate:findChild("btn_create")
        btn_create:addClickEventWithSound(function()
            if not self.playing then
                self:createRole(input:getText(), gender)
            end
        end)
        local nick = require("app.models.Common").getRandomNick(gender)
        input:setText(nick)
        local btn_random = self.pCreate:findChild('big_panel'):getChildByName("random")
        self.lastrandomname = ""
        btn_random:addClickEventWithSound(function()
            local nick = require("app.models.Common").getRandomNick(gender)
            self.lastrandomname = nick
            input:setText(nick)
        end)


    end)
    self:addChild(story)
    require("app.models.AudioManager").playmusic("BGM01.mp3", false)
end

function LoginScene:createRole(nick, gender)
    if nick == "" then
        cc.load("widget").PopWord:create(my.fmt "请输入昵称")
        return
    else
        local forbid = require('app.models.ForbiddenWords'):getInstance()
        local check = forbid:checkName(nick)
        if check then
            print(check)
            cc.load("widget").PopWord:create(my.fmt "存在非法字符，请重新输入")
            local input = self.pCreate:findChild('big_panel'):getChildByName("input_nick")
            input:setText(self.lastrandomname)
            return
        end
    end
    self.need_guide_battle = true
    require('app.models.login').createRole(gender, nick)
end

function LoginScene:loading()
    local User = require('app.models.user')
    if self.need_guide_battle then
        User.guide = 1
        require("app.models.ConfigManager").bt99999(cc.exports.App)
    else
        local Guide = require('app.views.guide.guide')
        local guide_id = Guide.hasForceGuide(User.guide)
        if guide_id then
            -- 还未完成强制引导
            User.guide = Guide.check()
        end
        require("app.models.AudioManager").stop()
        local view
        local tasks = {}
        local tick = 1
        --预加载config
        local configs = { "attendant", "item" }
        for _, v in ipairs(configs) do
            tasks[tick] = function()
                cc.load("utils").profile("登陆", "加载" .. v .. "配置文件0")
                require('app.models.ConfigManager').loadConfig(v)
                cc.load("utils").profile("登陆", "加载" .. v .. "配置文件1")
            end
            tick = tick + 1
        end
        --预加载图片
        --TODO: 图片名字获取
        local pics = {
            "sLobby.png", "outbg.jpg", "c5.png", "c6.jpg",
            "animation/c1_tex.png",
            "animation/c2_tex.png",
            "animation/c3_tex.png",
            "animation/c4_tex.png",
            "animation/duanzao_tex.png",
            "animation/gonghui_tex.png",
            "animation/renwu_tex.png",
            "animation/taixuhuanjing_tex.png",
            "animation/tongtianta_tex.png",
            "animation/xunbao_tex.png",
            "animation/zhaohun_tex.png",
        }
        for _, v in ipairs(pics) do
            tasks[tick] = function()
                cc.load("utils").profile("登陆", "加载" .. v .. "图片0")
                display.loadImage("creator/sLobby/" .. v .. ".png")
                cc.load("utils").profile("登陆", "加载" .. v .. "图片1")
            end
            tick = tick + 1
        end
        tasks[tick] = function() view.app_:enterScene("sLobby") end
        view = cc.exports.App:createView('sLoading', tasks)
        view:showWithScene()
    end
end

function LoginScene:fix()
    local popup = cc.load("widget").ConfirmBox:create(my.fmt("该操作会清除本地补丁，清除成功后请手动打开游戏"))
    popup:setPositiveBtn(function()
        local fu = cc.FileUtils:getInstance()
        fu:removeDirectory(fu:getWritablePath() .. 'hotfix')
        performWithDelay(self, function()
            if device.platform == "ios" then
                os.exit()
            else
                cc.Director:getInstance():endToLua()
            end
        end, 1)
    end, my.fmt '确定')
end

function LoginScene:openAnnouncement()
    if self.annoBtn:isVisible() then
        cc.exports.App:createView('sLogin_anno'):addToCenter(self)
    end
end

return LoginScene