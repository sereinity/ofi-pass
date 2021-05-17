# ofi-pass

Is a password promptor for [pass](http://zx2c4.com/projects/password-store/) that can use multiple typing engines and multiple password stores.

This project is inspired by [rofi-pass](https://github.com/carnager/rofi-pass) and the autotype feature should be compatible.


## Engines

Current prompt engines are:
- wofi (default and only one)
- rofi (soon as required to have multiple stores)

Typing engines:
- wtype (default, and only one for now)
- xdotool
- ydotool

| Engine name | Wayland | Xorg | Non US layout |
| xdotool     | ✗       | ✓    | ✓             |
| wtype       | ✓       | ✗    | ✓             |
| ydotool     | ✓       | ✓    | ✗             |


## TODOs

- Handle :otp
- Add multi-store (seams only possible via rofi to switch between stores)
- Find which typing tool to use? (sway only at first)
- Implement perfect merge of xdotool and wtype?
- Log if fail to parse password entry
- Handle multi-line password
- `#FILE=` an entryname to `pass show`


## OTP spec

A magic field `otp_method` defines a command line to run, ofi should type the result.
If no `otp_method` but the password starts with `otpauth://` it should call `pass-otp`
