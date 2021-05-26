# ofi-pass

Is a password promptor for [pass](http://zx2c4.com/projects/password-store/) that can use multiple typing engines and multiple password stores.

This project is inspired by [rofi-pass](https://github.com/carnager/rofi-pass) and the autotype feature should be compatible.


## Engines

Current prompt engines are:
- rofi (default, configurable via `OFI_TOOL` variable)
- wofi

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
- Handle multi-line password
- Handle `#FILE=` an entryname to `pass show`
- Add multi-store (seams only possible via rofi to switch between stores)
- Find which typing tool to use? (sway only at first)
- Check that rofi/wofi is installed before selecting it
- Implement perfect merge of xdotool and wtype?


## OTP spec

A magic field `otp_method` defines a command line to run, ofi should type the result.
If no `otp_method` but the password starts with `otpauth://` it should call `pass-otp`
