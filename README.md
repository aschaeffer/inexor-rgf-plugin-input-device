# Inexor Reactive Graph Flow

| Project | Module | Sub-Module | Functionality | Tests |
| --- | --- | --- | --- | --- |
| Reactive Graph Flow | Plugin | Input Device | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-input-device">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-input-device) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-input-device/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

This plugin enables input device event handling. It receives events from the evdev system
on linux and supports keys (keyboards, mouse buttons), relative axes (mouse x,y, scroll wheel),
absolute axes (joystick, touchpad), leds (numlock) and switches (power buttons).

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-input-device/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-input-device/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-input-device">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-input-device">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-input-device">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-input-device)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-input-device">](https://github.com/aschaeffer/inexor-rgf-plugin-input-device/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Type System

<img src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-input-device/main/docs/images/type_system.png">

#### Entity Types

| Name | Property | Data Type | Socket Type |
| --- | --- | --- | --- |
| InputDevice | name | string | output |
| | event | object | output |
| | physical_path | string | output |
| | driver_version | string | output |
| | vendor | number | output |
| | product | number | output |
| | version | number | output |
| InputDeviceKey | key | string | none |
| | key_code | number | none |
| | key_down | bool | output |
| InputDeviceLed | led | string | none |
| | led_type | number | none |
| | state | bool | output |
| InputDeviceRelativeAxis | relative_axis | string | none |
| | relative_axis_type | number | none |
| | state | number | output |
| InputDeviceAbsoluteAxis | absolute_axis | string | none |
| | absolute_axis_type | number | none |
| | state | number | output |
| InputDeviceSwitch | switch | string | none |
| | switch_type | number | none |
| | state | number | output |

#### Relation Types

| Name | Outbound Entity Type | Inbound Entity Type |
| --- | --- | --- |
| KeyEvent | InputDevice | InputDeviceKey |
| LedEvent | InputDevice | InputDeviceLed |
| RelativeAxisEvent | InputDevice | InputDeviceRelativeAxis |
| AbsoluteAxisEvent | InputDevice | InputDeviceAbsoluteAxis |
| SwitchEvent | InputDevice | InputDeviceSwitch |

#### Entity Behaviours

| Name | Description |
| --- | --- |
| InputDevice | Streams input events from evdev and sets the entity instance property `event` |

#### Relation Behaviours

| Name | Description |
| --- | --- |
| KeyEvent | Propagates input events and filters by event type (key event) and key code defined by the inbound entity instance |
| LedEvent | Propagates input events and filters by event type (led event) and led type defined by the inbound entity instance |
| RelativeAxisEvent | Propagates input events and filters by event type (relative axis event) and relative axis type defined by the inbound entity instance |
| AbsoluteAxisEvent | Propagates input events and filters by event type (absolute axis event) and absolute axis type defined by the inbound entity instance |
| SwitchEvent | Propagates input events and filters by event type (switch event) and switch type defined by the inbound entity instance |

### Input Device Types

#### Keys

A key changed state. A key, or button, is usually a momentary switch (in the circuit sense). It has two states: down,
or up. There are events for when keys are pressed (become down) and released (become up). There are also “key repeats”,
where multiple events are sent while a key is down.

| Name | Description |
| --- | --- |
| KEY_RESERVED | |
| KEY_ESC |  | 
| KEY_1 |  | 
| KEY_2 |  | 
| KEY_3 |  | 
| KEY_4 |  | 
| KEY_5 |  | 
| KEY_6 |  | 
| KEY_7 |  | 
| KEY_8 |  | 
| KEY_9 |  | 
| KEY_0 |  | 
| KEY_MINUS |  | 
| KEY_EQUAL |  | 
| KEY_BACKSPACE |  | 
| KEY_TAB |  | 
| KEY_Q |  | 
| KEY_W |  | 
| KEY_E |  | 
| KEY_R |  | 
| KEY_T |  | 
| KEY_Y |  | 
| KEY_U |  | 
| KEY_I |  | 
| KEY_O |  | 
| KEY_P |  | 
| KEY_LEFTBRACE |  | 
| KEY_RIGHTBRACE |  | 
| KEY_ENTER |  | 
| KEY_LEFTCTRL |  | 
| KEY_A |  | 
| KEY_S |  | 
| KEY_D |  | 
| KEY_F |  | 
| KEY_G |  | 
| KEY_H |  | 
| KEY_J |  | 
| KEY_K |  | 
| KEY_L |  | 
| KEY_SEMICOLON |  | 
| KEY_APOSTROPHE |  | 
| KEY_GRAVE |  | 
| KEY_LEFTSHIFT |  | 
| KEY_BACKSLASH |  | 
| KEY_Z |  | 
| KEY_X |  | 
| KEY_C |  | 
| KEY_V |  | 
| KEY_B |  | 
| KEY_N |  | 
| KEY_M |  | 
| KEY_COMMA |  | 
| KEY_DOT |  | 
| KEY_SLASH |  | 
| KEY_RIGHTSHIFT |  | 
| KEY_KPASTERISK |  | 
| KEY_LEFTALT |  | 
| KEY_SPACE |  | 
| KEY_CAPSLOCK |  | 
| KEY_F1 |  | 
| KEY_F2 |  | 
| KEY_F3 |  | 
| KEY_F4 |  | 
| KEY_F5 |  | 
| KEY_F6 |  | 
| KEY_F7 |  | 
| KEY_F8 |  | 
| KEY_F9 |  | 
| KEY_F10 |  | 
| KEY_NUMLOCK |  | 
| KEY_SCROLLLOCK |  | 
| KEY_KP7 |  | 
| KEY_KP8 |  | 
| KEY_KP9 |  | 
| KEY_KPMINUS |  | 
| KEY_KP4 |  | 
| KEY_KP5 |  | 
| KEY_KP6 |  | 
| KEY_KPPLUS |  | 
| KEY_KP1 |  | 
| KEY_KP2 |  | 
| KEY_KP3 |  | 
| KEY_KP0 |  | 
| KEY_KPDOT |  | 
| KEY_ZENKAKUHANKAKU |  | 
| KEY_102ND |  | 
| KEY_F11 |  | 
| KEY_F12 |  | 
| KEY_RO |  | 
| KEY_KATAKANA |  | 
| KEY_HIRAGANA |  | 
| KEY_HENKAN |  | 
| KEY_KATAKANAHIRAGANA |  | 
| KEY_MUHENKAN |  | 
| KEY_KPJPCOMMA |  | 
| KEY_KPENTER |  | 
| KEY_RIGHTCTRL |  | 
| KEY_KPSLASH |  | 
| KEY_SYSRQ |  | 
| KEY_RIGHTALT |  | 
| KEY_LINEFEED |  | 
| KEY_HOME |  | 
| KEY_UP |  | 
| KEY_PAGEUP |  | 
| KEY_LEFT |  | 
| KEY_RIGHT |  | 
| KEY_END |  | 
| KEY_DOWN |  | 
| KEY_PAGEDOWN |  | 
| KEY_INSERT |  | 
| KEY_DELETE |  | 
| KEY_MACRO |  | 
| KEY_MUTE |  | 
| KEY_VOLUMEDOWN |  | 
| KEY_VOLUMEUP |  | 
| KEY_POWER |  | 
| KEY_KPEQUAL |  | 
| KEY_KPPLUSMINUS |  | 
| KEY_PAUSE |  | 
| KEY_SCALE |  | 
| KEY_KPCOMMA |  | 
| KEY_HANGEUL |  | 
| KEY_HANJA |  | 
| KEY_YEN |  | 
| KEY_LEFTMETA |  | 
| KEY_RIGHTMETA |  | 
| KEY_COMPOSE |  | 
| KEY_STOP |  | 
| KEY_AGAIN |  | 
| KEY_PROPS |  | 
| KEY_UNDO |  | 
| KEY_FRONT |  | 
| KEY_COPY |  | 
| KEY_OPEN |  | 
| KEY_PASTE |  | 
| KEY_FIND |  | 
| KEY_CUT |  | 
| KEY_HELP |  | 
| KEY_MENU |  | 
| KEY_CALC |  | 
| KEY_SETUP |  | 
| KEY_SLEEP |  | 
| KEY_WAKEUP |  | 
| KEY_FILE |  | 
| KEY_SENDFILE |  | 
| KEY_DELETEFILE |  | 
| KEY_XFER |  | 
| KEY_PROG1 |  | 
| KEY_PROG2 |  | 
| KEY_WWW |  | 
| KEY_MSDOS |  | 
| KEY_COFFEE |  | 
| KEY_DIRECTION |  | 
| KEY_CYCLEWINDOWS |  | 
| KEY_MAIL |  | 
| KEY_BOOKMARKS |  | 
| KEY_COMPUTER |  | 
| KEY_BACK |  | 
| KEY_FORWARD |  | 
| KEY_CLOSECD |  | 
| KEY_EJECTCD |  | 
| KEY_EJECTCLOSECD |  | 
| KEY_NEXTSONG |  | 
| KEY_PLAYPAUSE |  | 
| KEY_PREVIOUSSONG |  | 
| KEY_STOPCD |  | 
| KEY_RECORD |  | 
| KEY_REWIND |  | 
| KEY_PHONE |  | 
| KEY_ISO |  | 
| KEY_CONFIG |  | 
| KEY_HOMEPAGE |  | 
| KEY_REFRESH |  | 
| KEY_EXIT |  | 
| KEY_MOVE |  | 
| KEY_EDIT |  | 
| KEY_SCROLLUP |  | 
| KEY_SCROLLDOWN |  | 
| KEY_KPLEFTPAREN |  | 
| KEY_KPRIGHTPAREN |  | 
| KEY_NEW |  | 
| KEY_REDO |  | 
| KEY_F13 |  | 
| KEY_F14 |  | 
| KEY_F15 |  | 
| KEY_F16 |  | 
| KEY_F17 |  | 
| KEY_F18 |  | 
| KEY_F19 |  | 
| KEY_F20 |  | 
| KEY_F21 |  | 
| KEY_F22 |  | 
| KEY_F23 |  | 
| KEY_F24 |  | 
| KEY_PLAYCD |  | 
| KEY_PAUSECD |  | 
| KEY_PROG3 |  | 
| KEY_PROG4 |  | 
| KEY_DASHBOARD |  | 
| KEY_SUSPEND |  | 
| KEY_CLOSE |  | 
| KEY_PLAY |  | 
| KEY_FASTFORWARD |  | 
| KEY_BASSBOOST |  | 
| KEY_PRINT |  | 
| KEY_HP |  | 
| KEY_CAMERA |  | 
| KEY_SOUND |  | 
| KEY_QUESTION |  | 
| KEY_EMAIL |  | 
| KEY_CHAT |  | 
| KEY_SEARCH |  | 
| KEY_CONNECT |  | 
| KEY_FINANCE |  | 
| KEY_SPORT |  | 
| KEY_SHOP |  | 
| KEY_ALTERASE |  | 
| KEY_CANCEL |  | 
| KEY_BRIGHTNESSDOWN |  | 
| KEY_BRIGHTNESSUP |  | 
| KEY_MEDIA |  | 
| KEY_SWITCHVIDEOMODE |  | 
| KEY_KBDILLUMTOGGLE |  | 
| KEY_KBDILLUMDOWN |  | 
| KEY_KBDILLUMUP |  | 
| KEY_SEND |  | 
| KEY_REPLY |  | 
| KEY_FORWARDMAIL |  | 
| KEY_SAVE |  | 
| KEY_DOCUMENTS |  | 
| KEY_BATTERY |  | 
| KEY_BLUETOOTH |  | 
| KEY_WLAN |  | 
| KEY_UWB |  | 
| KEY_UNKNOWN |  | 
| KEY_VIDEO_NEXT |  | 
| KEY_VIDEO_PREV |  | 
| KEY_BRIGHTNESS_CYCLE |  | 
| KEY_BRIGHTNESS_AUTO |  | 
| KEY_DISPLAY_OFF |  | 
| KEY_WWAN |  | 
| KEY_RFKILL |  | 
| KEY_MICMUTE |  | 
| BTN_0 |  | 
| BTN_1 |  | 
| BTN_2 |  | 
| BTN_3 |  | 
| BTN_4 |  | 
| BTN_5 |  | 
| BTN_6 |  | 
| BTN_7 |  | 
| BTN_8 |  | 
| BTN_9 |  | 
| BTN_LEFT |  | 
| BTN_RIGHT |  | 
| BTN_MIDDLE |  | 
| BTN_SIDE |  | 
| BTN_EXTRA |  | 
| BTN_FORWARD |  | 
| BTN_BACK |  | 
| BTN_TASK |  | 
| BTN_TRIGGER |  | 
| BTN_THUMB |  | 
| BTN_THUMB2 |  | 
| BTN_TOP |  | 
| BTN_TOP2 |  | 
| BTN_PINKIE |  | 
| BTN_BASE |  | 
| BTN_BASE2 |  | 
| BTN_BASE3 |  | 
| BTN_BASE4 |  | 
| BTN_BASE5 |  | 
| BTN_BASE6 |  | 
| BTN_DEAD |  | 
| BTN_SOUTH |  | 
| BTN_EAST |  | 
| BTN_C |  | 
| BTN_NORTH |  | 
| BTN_WEST |  | 
| BTN_Z |  | 
| BTN_TL |  | 
| BTN_TR |  | 
| BTN_TL2 |  | 
| BTN_TR2 |  | 
| BTN_SELECT |  | 
| BTN_START |  | 
| BTN_MODE |  | 
| BTN_THUMBL |  | 
| BTN_THUMBR |  | 
| BTN_TOOL_PEN |  | 
| BTN_TOOL_RUBBER |  | 
| BTN_TOOL_BRUSH |  | 
| BTN_TOOL_PENCIL |  | 
| BTN_TOOL_AIRBRUSH |  | 
| BTN_TOOL_FINGER |  | 
| BTN_TOOL_MOUSE |  | 
| BTN_TOOL_LENS |  | 
| BTN_TOOL_QUINTTAP |  | 
| BTN_TOUCH |  | 
| BTN_STYLUS |  | 
| BTN_STYLUS2 |  | 
| BTN_TOOL_DOUBLETAP |  | 
| BTN_TOOL_TRIPLETAP |  | 
| BTN_TOOL_QUADTAP |  | 
| BTN_GEAR_DOWN |  | 
| BTN_GEAR_UP |  | 
| KEY_OK |  | 
| KEY_SELECT |  | 
| KEY_GOTO |  | 
| KEY_CLEAR |  | 
| KEY_POWER2 |  | 
| KEY_OPTION |  | 
| KEY_INFO |  | 
| KEY_TIME |  | 
| KEY_VENDOR |  | 
| KEY_ARCHIVE |  | 
| KEY_PROGRAM |  | 
| KEY_CHANNEL |  | 
| KEY_FAVORITES |  | 
| KEY_EPG |  | 
| KEY_PVR |  | 
| KEY_MHP |  | 
| KEY_LANGUAGE |  | 
| KEY_TITLE |  | 
| KEY_SUBTITLE |  | 
| KEY_ANGLE |  | 
| KEY_ZOOM |  | 
| KEY_MODE |  | 
| KEY_KEYBOARD |  | 
| KEY_SCREEN |  | 
| KEY_PC |  | 
| KEY_TV |  | 
| KEY_TV2 |  | 
| KEY_VCR |  | 
| KEY_VCR2 |  | 
| KEY_SAT |  | 
| KEY_SAT2 |  | 
| KEY_CD |  | 
| KEY_TAPE |  | 
| KEY_RADIO |  | 
| KEY_TUNER |  | 
| KEY_PLAYER |  | 
| KEY_TEXT |  | 
| KEY_DVD |  | 
| KEY_AUX |  | 
| KEY_MP3 |  | 
| KEY_AUDIO |  | 
| KEY_VIDEO |  | 
| KEY_DIRECTORY |  | 
| KEY_LIST |  | 
| KEY_MEMO |  | 
| KEY_CALENDAR |  | 
| KEY_RED |  | 
| KEY_GREEN |  | 
| KEY_YELLOW |  | 
| KEY_BLUE |  | 
| KEY_CHANNELUP |  | 
| KEY_CHANNELDOWN |  | 
| KEY_FIRST |  | 
| KEY_LAST |  | 
| KEY_AB |  | 
| KEY_NEXT |  | 
| KEY_RESTART |  | 
| KEY_SLOW |  | 
| KEY_SHUFFLE |  | 
| KEY_BREAK |  | 
| KEY_PREVIOUS |  | 
| KEY_DIGITS |  | 
| KEY_TEEN |  | 
| KEY_TWEN |  | 
| KEY_VIDEOPHONE |  | 
| KEY_GAMES |  | 
| KEY_ZOOMIN |  | 
| KEY_ZOOMOUT |  | 
| KEY_ZOOMRESET |  | 
| KEY_WORDPROCESSOR |  | 
| KEY_EDITOR |  | 
| KEY_SPREADSHEET |  | 
| KEY_GRAPHICSEDITOR |  | 
| KEY_PRESENTATION |  | 
| KEY_DATABASE |  | 
| KEY_NEWS |  | 
| KEY_VOICEMAIL |  | 
| KEY_ADDRESSBOOK |  | 
| KEY_MESSENGER |  | 
| KEY_DISPLAYTOGGLE |  | 
| KEY_SPELLCHECK |  | 
| KEY_LOGOFF |  | 
| KEY_DOLLAR |  | 
| KEY_EURO |  | 
| KEY_FRAMEBACK |  | 
| KEY_FRAMEFORWARD |  | 
| KEY_CONTEXT_MENU |  | 
| KEY_MEDIA_REPEAT |  | 
| KEY_10CHANNELSUP |  | 
| KEY_10CHANNELSDOWN |  | 
| KEY_IMAGES |  | 
| KEY_DEL_EOL |  | 
| KEY_DEL_EOS |  | 
| KEY_INS_LINE |  | 
| KEY_DEL_LINE |  | 
| KEY_FN |  | 
| KEY_FN_ESC |  | 
| KEY_FN_F1 |  | 
| KEY_FN_F2 |  | 
| KEY_FN_F3 |  | 
| KEY_FN_F4 |  | 
| KEY_FN_F5 |  | 
| KEY_FN_F6 |  | 
| KEY_FN_F7 |  | 
| KEY_FN_F8 |  | 
| KEY_FN_F9 |  | 
| KEY_FN_F10 |  | 
| KEY_FN_F11 |  | 
| KEY_FN_F12 |  | 
| KEY_FN_1 |  | 
| KEY_FN_2 |  | 
| KEY_FN_D |  | 
| KEY_FN_E |  | 
| KEY_FN_F |  | 
| KEY_FN_S |  | 
| KEY_FN_B |  | 
| KEY_BRL_DOT1 |  | 
| KEY_BRL_DOT2 |  | 
| KEY_BRL_DOT3 |  | 
| KEY_BRL_DOT4 |  | 
| KEY_BRL_DOT5 |  | 
| KEY_BRL_DOT6 |  | 
| KEY_BRL_DOT7 |  | 
| KEY_BRL_DOT8 |  | 
| KEY_BRL_DOT9 |  | 
| KEY_BRL_DOT10 |  | 
| KEY_NUMERIC_0 |  | 
| KEY_NUMERIC_1 |  | 
| KEY_NUMERIC_2 |  | 
| KEY_NUMERIC_3 |  | 
| KEY_NUMERIC_4 |  | 
| KEY_NUMERIC_5 |  | 
| KEY_NUMERIC_6 |  | 
| KEY_NUMERIC_7 |  | 
| KEY_NUMERIC_8 |  | 
| KEY_NUMERIC_9 |  | 
| KEY_NUMERIC_STAR |  | 
| KEY_NUMERIC_POUND |  | 
| KEY_CAMERA_FOCUS |  | 
| KEY_WPS_BUTTON |  | 
| KEY_TOUCHPAD_TOGGLE |  | 
| KEY_TOUCHPAD_ON |  | 
| KEY_TOUCHPAD_OFF |  | 
| KEY_CAMERA_ZOOMIN |  | 
| KEY_CAMERA_ZOOMOUT |  | 
| KEY_CAMERA_UP |  | 
| KEY_CAMERA_DOWN |  | 
| KEY_CAMERA_LEFT |  | 
| KEY_CAMERA_RIGHT |  | 
| KEY_ATTENDANT_ON |  | 
| KEY_ATTENDANT_OFF |  | 
| KEY_ATTENDANT_TOGGLE |  | 
| KEY_LIGHTS_TOGGLE |  | 
| BTN_DPAD_UP |  | 
| BTN_DPAD_DOWN |  | 
| BTN_DPAD_LEFT |  | 
| BTN_DPAD_RIGHT |  | 
| KEY_ALS_TOGGLE |  | 
| KEY_BUTTONCONFIG |  | 
| KEY_TASKMANAGER |  | 
| KEY_JOURNAL |  | 
| KEY_CONTROLPANEL |  | 
| KEY_APPSELECT |  | 
| KEY_SCREENSAVER |  | 
| KEY_VOICECOMMAND |  | 
| KEY_BRIGHTNESS_MIN |  | 
| KEY_BRIGHTNESS_MAX |  | 
| KEY_KBDINPUTASSIST_PREV |  | 
| KEY_KBDINPUTASSIST_NEXT |  | 
| KEY_KBDINPUTASSIST_PREVGROUP |  | 
| KEY_KBDINPUTASSIST_NEXTGROUP |  | 
| KEY_KBDINPUTASSIST_ACCEPT |  | 
| KEY_KBDINPUTASSIST_CANCEL |  | 
| BTN_TRIGGER_HAPPY1 |  | 
| BTN_TRIGGER_HAPPY2 |  | 
| BTN_TRIGGER_HAPPY3 |  | 
| BTN_TRIGGER_HAPPY4 |  | 
| BTN_TRIGGER_HAPPY5 |  | 
| BTN_TRIGGER_HAPPY6 |  | 
| BTN_TRIGGER_HAPPY7 |  | 
| BTN_TRIGGER_HAPPY8 |  | 
| BTN_TRIGGER_HAPPY9 |  | 
| BTN_TRIGGER_HAPPY10 |  | 
| BTN_TRIGGER_HAPPY11 |  | 
| BTN_TRIGGER_HAPPY12 |  | 
| BTN_TRIGGER_HAPPY13 |  | 
| BTN_TRIGGER_HAPPY14 |  | 
| BTN_TRIGGER_HAPPY15 |  | 
| BTN_TRIGGER_HAPPY16 |  | 
| BTN_TRIGGER_HAPPY17 |  | 
| BTN_TRIGGER_HAPPY18 |  | 
| BTN_TRIGGER_HAPPY19 |  | 
| BTN_TRIGGER_HAPPY20 |  | 
| BTN_TRIGGER_HAPPY21 |  | 
| BTN_TRIGGER_HAPPY22 |  | 
| BTN_TRIGGER_HAPPY23 |  | 
| BTN_TRIGGER_HAPPY24 |  | 
| BTN_TRIGGER_HAPPY25 |  | 
| BTN_TRIGGER_HAPPY26 |  | 
| BTN_TRIGGER_HAPPY27 |  | 
| BTN_TRIGGER_HAPPY28 |  | 
| BTN_TRIGGER_HAPPY29 |  | 
| BTN_TRIGGER_HAPPY30 |  | 
| BTN_TRIGGER_HAPPY31 |  | 
| BTN_TRIGGER_HAPPY32 |  | 
| BTN_TRIGGER_HAPPY33 |  | 
| BTN_TRIGGER_HAPPY34 |  | 
| BTN_TRIGGER_HAPPY35 |  | 
| BTN_TRIGGER_HAPPY36 |  | 
| BTN_TRIGGER_HAPPY37 |  | 
| BTN_TRIGGER_HAPPY38 |  | 
| BTN_TRIGGER_HAPPY39 |  | 
| BTN_TRIGGER_HAPPY40 |  | 

#### LEDs

An LED was toggled.

| Name | Description |
| --- | --- |
| LED_NUML  |  |  
| LED_CAPSL  |  |  
| LED_SCROLLL  |  |  
| LED_COMPOSE  |  |  
| LED_KANA  |  |  
| LED_SLEEP  | Stand-by |
| LED_SUSPEND  |  |  
| LED_MUTE  |  |  
| LED_MISC  | Generic indicator |  
| LED_MAIL  | Message waiting |  
| LED_CHARGING  | External power connected |  

#### Relative Axes

Movement on a relative axis. There is no absolute coordinate frame, just the fact that there was a change of a
certain amount of units. Used for things like mouse movement or scroll wheels.

| Name | Description |
| --- | --- |
| REL_X  |  |  
| REL_Y  |  |  
| REL_Z  |  |  
| REL_RX  |  |  
| REL_RY  |  |  
| REL_RZ  |  |  
| REL_HWHEEL  |  |  
| REL_DIAL  |  |  
| REL_WHEEL  |  |  
| REL_MISC  |  |  
| REL_RESERVED  |  |  
| REL_WHEEL_HI_RES  |  |  
| REL_HWHEEL_HI_RES  |  |  

#### Absolute Axes

Movement on an absolute axis. Used for things such as touch events and joysticks.

| Name | Description |
| --- | --- |
| ABS_X  |  |  
| ABS_Y  |  |  
| ABS_Z  |  |  
| ABS_RX  |  |  
| ABS_RY  |  |  
| ABS_RZ  |  |  
| ABS_THROTTLE  |  |  
| ABS_RUDDER  |  |  
| ABS_WHEEL  |  |  
| ABS_GAS  |  |  
| ABS_BRAKE  |  |  
| ABS_HAT0X  |  |  
| ABS_HAT0Y  |  |  
| ABS_HAT1X  |  |  
| ABS_HAT1Y  |  |  
| ABS_HAT2X  |  |  
| ABS_HAT2Y  |  |  
| ABS_HAT3X  |  |  
| ABS_HAT3Y  |  |  
| ABS_PRESSURE  |  |  
| ABS_DISTANCE  |  |  
| ABS_TILT_X  |  |  
| ABS_TILT_Y  |  |  
| ABS_TOOL_WIDTH  |  |  
| ABS_VOLUME  |  |  
| ABS_MISC  |  |  
| ABS_MT_SLOT  | MT slot being modified |  
| ABS_MT_TOUCH_MAJOR  | Major axis of touching ellipse |  
| ABS_MT_TOUCH_MINOR  | Minor axis (omit if circular) |  
| ABS_MT_WIDTH_MAJOR  | Major axis of approaching ellipse |  
| ABS_MT_WIDTH_MINOR  | Minor axis (omit if circular) |  
| ABS_MT_ORIENTATION  | Ellipse orientation |  
| ABS_MT_POSITION_X  | Center X touch position |  
| ABS_MT_POSITION_Y  | Center Y touch position |  
| ABS_MT_TOOL_TYPE  | Type of touching device |  
| ABS_MT_BLOB_ID  | Group a set of packets as a blob |  
| ABS_MT_TRACKING_ID  | Unique ID of the initiated contact |  
| ABS_MT_PRESSURE  | Pressure on contact area |  
| ABS_MT_DISTANCE  | Contact over distance |  
| ABS_MT_TOOL_X  | Center X tool position |  
| ABS_MT_TOOL_Y  | Center Y tool position |  

#### Switches

Change in a switch value. Switches are boolean conditions and usually correspond to a toggle switch of some kind
in hardware.

| Name | Description |
| --- | --- |
| SW_LID | set = lid shut |
| SW_TABLET_MODE | set = tablet mode |
| SW_HEADPHONE_INSERT | set = inserted |
| SW_RFKILL_ALL | rfkill master switch, type ‘any’ |
| SW_MICROPHONE_INSERT | set = inserted |
| SW_DOCK | set = plugged into doc |
| SW_LINEOUT_INSERT | set = inserted |
| SW_JACK_PHYSICAL_INSERT | set = mechanical switch set |
| SW_VIDEOOUT_INSERT | set = inserted |
| SW_CAMERA_LENS_COVER | set = lens covered |
| SW_KEYPAD_SLIDE | set = keypad slide out |
| SW_FRONT_PROXIMITY | set = front proximity sensor active |
| SW_ROTATE_LOCK | set = rotate locked/disabled |
| SW_LINEIN_INSERT | set = inserted |
| SW_MUTE_DEVICE | set = device disabled |
| SW_PEN_INSERTED | set = pen inserted |
| SW_MACHINE_COVER | set = cover closed |


### TODO

- [ ] Virtual Keyboard
  - [ ] Create a virtual device (useful for embedded devices without a keyboard)
  - [ ] Send key events via the virtual device

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

| | | |
| --- | --- | --- |
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-logical/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
