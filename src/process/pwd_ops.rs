// 引入SliceRandom trait
// 其中的choose()方法可以返回对切片中随机元素的引用
// 如果切片为空,则返回None
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_,./[]{}~()-+=?<>";

pub fn process_pwd(
    len: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    // 根据下面的4个if,决定了chars能包含哪几类的字符
    // 尽管每种都可能提前塞了字符在password里,但是shuffle方法可以随机打乱
    // 因此并不会让前面的几个字符类型固定
    if upper {
        //pub fn extend_from_slice(&mut self, other: &[T])
        // 克隆并将切片中的所有元素追加到Vec
        chars.extend_from_slice(UPPER);
        //fn choose<R>(&self, rng: &mut R) -> Option<&Self::Item>
        //where R: Rng + ?Sized
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    if password.is_empty() {
        return Err(anyhow::anyhow!("请选择至少一种字符类型!"));
    }

    if len < password.len() as u8 {
        return Err(anyhow::anyhow!("-L需要更大的值,或者减少字符类型!"));
    }

    for _ in 0..(len - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    // "洗牌",随机打乱
    password.shuffle(&mut rng);

    println!("{}", String::from_utf8(password)?);

    Ok(())
}
