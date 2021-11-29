
local cls = class(..., cc.load("mvc").Scene(...))
local mvc=cc.load [[mvc]]
local Common = require("app.models.Common")
local User = require("app.models.user")
local ActModel = require("app.views.activity.activity")
local pb_shared = pb:require("Shared")
local CM = require("app.models.ConfigManager")
local RM = require("app.models.ResManager")
local Conjure = require("app.models.Conjure")

function cls:ctor(_app, _name)
    self._needItem=2313 --上古卷轴
    self._conjureCount={[15]=20,[16]=40,[17]=60} --分别对应S,SS,SSS
    self._aptitude={[15]="s",[16]="ss",[17]="sss"} --分别对应S,SS,SSS

    self._canvas = self:getChildByName("Canvas")
    self._root= self._canvas:getChildByName("root")
    local top=self._root:getChildByName("top")
    local right=self._root:getChildByName("right")
    local middle=self._root:getChildByName("middle")
    local foot=self._root:getChildByName("foot")
    top:setContentSize(cc.size(display.width,57))
    top:getChildByName("title_bg"):setScaleX(display.width / 1280)
    top:setPosition(cc.p(display.width/2,display.height-top:getContentSize().height/2))

    self._rightPosition=right:getPositionX()
    self._topPosition=top:getPositionY()
    self._foot=foot
    self._top=top
    self._right=right
    self._actType = pb_shared.AT_LAIR_CONJURE

    local function btnHandler(node, callback, name, sound)
        local btn = node
        if name then
            btn = cc.utils.findChild(node, name)
        end
        btn:addClickEventWithSound(handler(self, callback), sound or Audio.btn_normal)
        return btn
    end
    btnHandler(top, self.back,"btn_close",Audio.btn_normal)
    btnHandler(top, self.goBuy, "btn_buy", Audio.btn_change)
    btnHandler(foot, self.conjure,"btn_one",Audio.btn_normal)
    btnHandler(foot, self.conjureTen,"btn_ten",Audio.btn_normal)
    btnHandler(right, self.goScroll, "btn_getscroll", Audio.btn_change)
    btnHandler(right, self.goBuy, "btn_buy", Audio.btn_change)
    btnHandler(right, self.goStagereward, "btn_stagereward", Audio.btn_change)
    btnHandler(right, self.goShuoming, "btn_shuoming", Audio.btn_change)
    self._cfg=CM.loadConfig("Cave")
    self._cfgAttendant = CM.loadConfig("attendant")

    local middle=self._root:getChildByName("middle")
    btnHandler(middle:getChildByName("select"), self.goReplaceatt, "btn", Audio.btn_change)
    Common.setTextureScale(middle:getChildByName("tip"):getChildByName('quality_bg'), RM.getHeadIconQualityPath("ur"))
    Common.setTextureScale(middle:getChildByName("tip"):getChildByName('quality'), RM.getHeadFrameQualityPath("ur"))
    Common.setTextureScale(middle:getChildByName("select"):getChildByName("hero"):getChildByName('quality_bg'), RM.getHeadIconQualityPath("ur"))
    local icon = top:getChildByName("money"):getChildByName("icon")
    icon:addLuaCom("aItemTips")
    icon.item_id =self._needItem
    icon:loadTextureNormal(RM.getItemIconPath(self._needItem))
    --设置奖励的地方
    for i=1,11 do
        local item_id=i==11 and self._cfg[1].reward_3[1] or self._cfg[1].reward_1[(i-1)*3+1]
        local itemNode=middle:getChildByName("item"..i)
        itemNode:addLuaCom("aItemTips")
        if item_id then
            itemNode.item_id = item_id
            local count=i==11 and self._cfg[1].reward_3[2] or self._cfg[1].reward_1[(i-1)*3+2]
            itemNode:loadTextureNormal(RM.getItemQualityBgPath(item_id))
            RM.setItemIcon(itemNode:getChildByName("icon_item"), item_id)
            itemNode:getChildByName("num"):getChildByName("label"):setString(User.replaceNumber(count))
            itemNode:show()
        else
            itemNode:hide()
        end
    end
end

function cls:onEnter()
    --获取曾经拥有的侍从,给心愿侍从用
    Conjure.getHead(function(data)
        self.canselecteddata = data
    end)

    local rightHidePosition=display.right+50
    local topHidePosition=display.height+50
    self._top:setPositionY(topHidePosition)
    self._right:setPositionX(rightHidePosition)
    self._right:runAction(cc.MoveBy:create(0.3,cc.p(self._rightPosition-rightHidePosition,0)))
    self._top:runAction(cc.MoveBy:create(0.3,cc.p(0,self._topPosition-topHidePosition)))

    self._data = ActModel[self._actType]
    self._idx=self._data.activity.idx
    self._shengyu=self._data.extra_value --剩余出英雄次数
    self._hero=self._data.value --心愿侍从
    self._hunyucishu=self._data.extra2_value --剩余可用魂玉次数
    self._huntCount=self._data.extra3_value  --活动期间已探宝次数

    local foot=self._root:getChildByName("foot")
     --日期
     local t = os.date("*t")
     local t1 = os.date("*t", self._data.activity["start"])  --开始
     local t2 = os.date("*t", self._data.activity["end"])  --结束
     local real_end = os.time(t2)  --实际结束时间(不包含领奖)
     local real_start=os.time(t1)
     self.real_end=real_end
     foot:getChildByName("label"):setString(my.fmt("活动时间:{0}",my.fmt("{0}月{1}日", t1.month, t1.day).."-"..my.fmt("{0}月{1}日",t2.month, t2.day)))
     
     RM.setItemIcon(foot:getChildByName("one_icon"),self._needItem)
     RM.setItemIcon(foot:getChildByName("ten_icon"),self._needItem)
     foot:getChildByName("one_count"):setString(20)
     foot:getChildByName("ten_count"):setString(180)
    --积分
    local pmoney = self._top:getChildByName("money")
    pmoney:getChildByName("label"):setString(User.replaceNumber(User.getItemCount(self._needItem),100000))
    RM.setItemIcon(pmoney:getChildByName("icon"):getRendererNormal(), self._needItem)
    self:refreshItemCount()
    self:setHero()
    self:checkRedPoint()
end


function cls:checkRedPoint()
    --阶段奖励红点
    --活动红点
    local stageRedPoint=false
    local activityRedPoint=false
    local value_items=self._data.value_items.item
    for i=1,#value_items do
        local v=value_items[i]
        if v.status==1 then
            stageRedPoint=true
            break
        end
    end

    local items=self._data.items.item
    for i=1,#items do
        local v=items[i]
        if v.status==1 then
            activityRedPoint=true
            break
        end
    end
    self._right:getChildByName("btn_getscroll"):getChildByName("dot"):setVisible(activityRedPoint)
    self._right:getChildByName("btn_stagereward"):getChildByName("dot"):setVisible(stageRedPoint)
    require('app.models.RedPoints'):getInstance():set("sLair", activityRedPoint or stageRedPoint)
end

function cls:goScroll()
    ActModel.getActivityState(self._actType, function()
        local view = self.app_:createView("sLairhunt_getscroll")
        view:addToCenter(self)
        view.onExitCallback_ = function()
            self:refreshItemCount()
            self:checkRedPoint()
        end
    end)
end

--探宝一次
function cls:conjure()
    if self._hero and self._hero>0 then
        local item_count=User.getItemCount(self._needItem)
        local item_id=item_count>=20 and self._needItem or ENUM.MONEY_TYPE.DIAMOND
        local tanbao=function ()
            ActModel.lairConjure({item_id=item_id,item_count=1},function (list,count)
                self._shengyu=count
                self._hunyucishu=ActModel[self._actType].extra2_value
                self._huntCount=ActModel[self._actType].extra3_value
                self:setHero()
                self:refreshItemCount()
                cc.load("widget").PopItem:create(list)
            end)
        end
        if item_id==ENUM.MONEY_TYPE.DIAMOND then
            local pop = cc.load("widget").ConfirmBox:create(cc.load("utils"):makeXML(my.fmt("上古卷轴不足,是否使用{0}{2}进行探宝？\n")..my.fmt("(今日剩余魂玉探宝次数:{0}次)",self._hunyucishu), { item = ENUM.MONEY_TYPE.DIAMOND, count = 400}), 2)
            pop:setPositiveBtn(function()
                local fail_reason
                if self._hunyucishu<1 then
                    fail_reason=my.fmt("魂玉探宝剩余次数不足,无法探宝.")
                end
                if User.getItemCount(ENUM.MONEY_TYPE.DIAMOND)<400 then
                    fail_reason=my.fmt("魂玉不足")
                end
                if fail_reason then
                    cc.load("widget").PopWord:create(fail_reason)
                else
                    tanbao()
                end
                pop:close()
            end, my.fmt "确定")
        else
            tanbao()
        end
    else
        cc.load("widget").PopWord:create(my.fmt("请先选择您的心愿侍从"))
    end
end

--探宝十次
function cls:conjureTen()
    if self._hero and self._hero>0 then
        local item_count=User.getItemCount(self._needItem)
        local item_id=item_count>=180 and self._needItem or ENUM.MONEY_TYPE.DIAMOND
        local tanbao=function ()
            ActModel.lairConjure({item_id=item_id,item_count=10},function (list,count)
                self._shengyu=count
                self._hunyucishu=ActModel[self._actType].extra2_value
                self._huntCount=ActModel[self._actType].extra3_value
                self:setHero()
                self:refreshItemCount()
                cc.load("widget").PopItem:create(list)
            end)
        end
        if item_id==ENUM.MONEY_TYPE.DIAMOND then
            local pop = cc.load("widget").ConfirmBox:create(cc.load("utils"):makeXML(my.fmt("上古卷轴不足,是否使用{0}{2}进行十次探宝？\n")..my.fmt("(今日剩余魂玉探宝次数:{0}次)",self._hunyucishu), { item = ENUM.MONEY_TYPE.DIAMOND, count = 3600}), 2)
            pop:setPositiveBtn(function()
                local fail_reason
                if self._hunyucishu<10 then
                    fail_reason=my.fmt("魂玉探宝剩余次数不足,无法探宝.")
                end
                if User.getItemCount(ENUM.MONEY_TYPE.DIAMOND)<3600 then
                    fail_reason=my.fmt("魂玉不足")
                end
                if fail_reason then
                    cc.load("widget").PopWord:create(fail_reason)
                else
                    tanbao()
                end
                pop:close()
            end, my.fmt "确定")
        else
            tanbao()
        end
    else
        cc.load("widget").PopWord:create(my.fmt("请先选择您的心愿侍从"))
    end
end

function cls:setHero()
    local middle=self._root:getChildByName("middle")
    local btn_select = middle:getChildByName("select")
    local heroinfo=btn_select:getChildByName("hero")
    middle:getChildByName("huntcount"):setXML(my.fmt("当前活动累计探宝 <font color='#46EC24'>{0}</font> 次",self._huntCount))
    if self._hero and self._hero>0 then
        local aptitude=tostring(self._cfgAttendant[self._hero].aptitude)
        if btn_select:getChildByName("tip") then
            btn_select:getChildByName("tip"):removeFromParent()
        end
        local childrens=heroinfo:getChildByName("pinzhi"):getChildren()
        for _,v in pairs(childrens) do
            if v:getName()==aptitude then
                v:show()
            else
                v:hide()
            end
        end
        local headSkin, animSkin = User.getReplaceHeroSkin({attendant={id=self._hero}})
        Common.setTextureScale(middle:getChildByName("tip"):getChildByName('hero_head'), RM.getHeroHeadIconPath(headSkin))
        Common.setTextureScale(heroinfo:getChildByName('hero_head'), RM.getHeroHeadIconPath(headSkin))
        middle:getChildByName("tip"):getChildByName("label"):setXML(my.fmt("剩余<font color='#13de2b'>{0}</font>次必出心愿侍从",self._shengyu>0 and self._shengyu or 1))
    else
            performWithDelay(self, function()
                    if not btn_select:getChildByName("tip") then
                        local animation = mvc.loadRoot("wGuide")
                        animation:setName("tip")
                        local node = animation:getChildByName("node1")
                        node:hide()
                        node = animation:getChildByName("node2")
                        node:show():getChildByName("label"):setString(my.fmt "选择心愿侍从")
                        local anim = node:getChildByName("yindao_ske")
                        anim:addDBEventListener(ccdb.EventObject.LOOP_COMPLETE, function(object)
                            if object.animationState:getName() == "Animation1" then
                                anim:getAnimation():play("Animation2")
                            else
                                anim:getAnimation():play("Animation1")
                            end
                        end)
                        animation:addTo(btn_select):move(60, 60)
                    end
            end, 0.1)
    end
    middle:getChildByName("tip"):setVisible(self._hero and self._hero>0)
    heroinfo:setVisible(self._hero and self._hero>0)
end

function cls:refreshItemCount()
    local item_count=User.getItemCount(self._needItem)
    self._top:getChildByName("money"):getChildByName("label"):setString(User.replaceNumber(item_count,10000))
end


function cls:goShuoming(sender)
    cc.load("widget").TipRules:create(nil, my.fmt [[1.龙穴探宝可自从已拥有的侍从中自选一名侍从做为心愿侍从。
2.S/SS/SSS侍从必出次数分别需要20/40/60次探宝。
3.每次十连必出红色侍从，包括：普通红侍从，精英红侍从，核心红侍从，心愿侍从。
4.中途可更换心愿侍从，更换后剩余次数会根据当前心愿侍从的需求数变化。
5.第110次，210次，310次…必出核心红侍从。
6.活动期间每天玩家可以用魂玉探宝20次，用上古卷轴探宝不限次数。]], nil, sender)
end

--选择侍从
function cls:goReplaceatt()
    local data={}
    local aptitude={}
    local conjureCount=0
    if self._hero and self._hero>0 then
        conjureCount=self._conjureCount[self._cfgAttendant[self._hero].aptitude]-self._shengyu
    end
    for k,v in pairs(self._conjureCount) do
        if v>conjureCount then
            aptitude[k]=true
        end
    end
    for k, v in pairs(self.canselecteddata) do
        if self._cfgAttendant[v].id > 103 and self._cfgAttendant[v].quality == "ur" and v ~= self._hero then
            table.insert(data, v)
        end
    end
    local node = self.app_:createView("sLairhunt_replaceatt", self, data, (self._hero and self._hero>0) and self._aptitude[self._cfgAttendant[self._hero].aptitude] or "s",aptitude, function(choose)
        if choose.aid~=self._hero then
            ActModel.lairConjureSet(choose.aid,function (shenyu)
                self._hero=choose.aid
                self._shengyu=shenyu
                self:setHero() 
            end)
        end
    end)
    node:addToCenter(self)
end
--限时商店
function cls:goBuy()
    local view = self.app_:createView("sActivity3",pb_shared.AT_LAIR_CONJURE_GIFT)
    view:addToCenter(self)
    view.onExitCallback_ = function()
        local pmoney = self._top:getChildByName("money")
        self._hasItemCount=User.getItemCount(self._needItem)
        pmoney:getChildByName("label"):setString(User.replaceNumber(self._hasItemCount,100000))
    end
end
--阶段奖励
function cls:goStagereward()
    if os.time()>self.real_end then
        cc.load("widget").PopWord:create(my.fmt("活动已结束"))
        return
    end
    ActModel.getActivityState(self._actType, function()
        local view =self.app_:createView("sLairhunt_stagereward")
        view:addToCenter(self)
        view.onExitCallback_ = function()
            self:checkRedPoint()
        end
    end)
end

function cls:back(sender)
    self.app_:popScene()
end

return cls
