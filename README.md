<h1 align="center">Hi there</a> 
<img src="https://github.com/blackcater/blackcater/raw/main/images/Hi.gif" height="32"/></h1>
<h3 align="center">Make your custom qr with QrMeow</h3>

![Typing SVG](https://readme-typing-svg.herokuapp.com?color=%2336BCF7&duration=10000&center=true&width=1000&lines=red+QrCode+custom+colorful+rgba+easy+.unwrap()+green+ILOVERUST+blue+QRCODE)
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
    .resolution(/* u32 */)
    .style(/* QrStyle:: */)
    .color(/* [u8; 4] */)
    .bgcolor(/* [u8; 4] */)

// version: 1 - 40
// style: Default, Rounded, Half
// color / bgcolor: rgba --> example: [0, 0, 0, 255] <-- black
```

**Gallery**:

<img 
    src="https://github.com/Tester0521/qr_meow/blob/master/assets/12.png" 
    width="200" height="200" alt="style Default"
/>
<img 
    src="https://github.com/Tester0521/qr_meow/blob/master/assets/123.png" 
    width="200" height="200" caption="style Rounded" alt="style Rounded"
/>
<img 
    src="https://github.com/Tester0521/qr_meow/blob/master/assets/1234.png" 
    width="200" height="200" caption="style Half" alt="style Half"
/>




