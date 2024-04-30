use std::arch::asm;

pub fn getSysInfo() -> Vec<String> {
    let mut buf = [0_u8; 1080]; // sizeof(struct utsname)

    unsafe {
	   asm!(
           "mov rax, 63", // uname
	       "syscall",
           in("rdi") buf.as_mut_ptr(),
        );
    }

    let mut valueBuf: Vec<String> = vec![];

    let mut lastWasNull = false;
    let mut curStr = String::new();
    for n in buf {
        if n == 0 && !lastWasNull {
            valueBuf.push(curStr);
            curStr = String::new();
        }

        if n == 0{
            lastWasNull = true;
            continue;
        }

        lastWasNull = false;
        curStr.push(char::from_u32(n.into()).unwrap());
    }

    valueBuf

}
