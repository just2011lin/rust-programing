pub fn main_no_value_enum() {
    #[derive(Debug)]
    enum Boolean {
        True,
        False,
    }
    let a = Boolean::True;
    // 指定类型
    let b: Boolean = Boolean::False;
    println!("a = {a:?}, b = {b:?}");
}

pub fn main_with_value_enum() {
    #[derive(Debug)]
    // 饮料
    enum Drink {
        // 普通水，带多少毫升数据
        Water(u32),
        // 可乐
        Cole {
            // 是否含糖
            sugar: bool,
            // 价格
            price: f64,
            // 含量
            milliliter: u32,
        },
        // 热水，毫升，温度
        HotWater(u32, i8),
    }
    // 一百毫升的水
    let a = Drink::Water(100);
    // 330毫升售价2.5元的无糖可乐
    let b = Drink::Cole {
        sugar: false,
        price: 2.5,
        milliliter: 330,
    };
    // 100毫升55度的热水
    let c: Drink = Drink::HotWater(100, 55);
    println!("a = {a:?}, b = {b:?}, c = {c:?}");

    match b {
        Drink::Water(ml) => println!("{ml}毫升的水"),
        Drink::Cole {
            sugar,
            price,
            milliliter,
        } => {
            let suguar_text = if sugar { "含糖" } else { "无糖" };
            println!("{milliliter}毫升售价{price}元的{suguar_text}可乐");
        }
        Drink::HotWater(ml, temperature) => {
            println!("{temperature}度{ml}毫升的热水")
        }
    }
}
