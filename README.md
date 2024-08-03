<h1 align="center">Hi there, I'm <a href="https://github.com/Tester0521" target="_blank">qitteenn</a> 
<img src="https://github.com/blackcater/blackcater/raw/main/images/Hi.gif" height="32"/></h1>
<h3 align="center">Make your custom qr with QrMeow</h3>

[![Typing SVG](https://readme-typing-svg.herokuapp.com?color=%2336BCF7&duration=10000&center=true&width=1000&lines=red+QrCode+custom+colorful+rgba+easy+.unwrap()+green+ILOVERUST+blue+QRCODE)](https://git.io/typing-svg)

**adding**:
```bash
cargo add qrMeow // not available now...sooon
```

**usage**:
```Rust
use qrMeow::{QrCode, QrStyle};

QrCode::new()
    .version(/* i16 */)
    .data(/* &str */)
    .style(/* QrStyle:: */)
    .resolution(/* u32 */)

// version: 1 - 40
// style: Default, Rounded
```

**Gallery**:
![qr 1](https://github.com/Tester0521/qr_meow/blob/master/assets/123.png)

