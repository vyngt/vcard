macro_rules! create_function {
    ($my_func_name:ident) => {
        fn $my_func_name(name: &str) -> String {
            format!("{:?} {}", stringify!($my_func_name), name)
        }
    };
}

fn world(name: &str) -> String {
    format!("{} {}", "world", name)
}

create_function!(vynt);
create_function!(hello);

// My Vec!

macro_rules! vynt_vec {
    ($( $x:expr ),*) => {
        {
            let mut vector = Vec::new();
            $(
                vector.push($x);
            )*
            vector
        }
    };
}

macro_rules! S {
    ($e:expr) => {
        String::from($e)
    };
}

macro_rules! talk {
    ($($i:ident)*) => {
        {
            let mut temp_str = String::from("");
            $(
                temp_str.push_str(stringify!($i));
                temp_str.push_str(" ");
            )*
            println!("{}", temp_str);
        }
    };
}

fn main() {
    let v = vynt("18 Tuoi");
    let greet = hello("World");
    println!("{v}\n{greet}");
    println!("{}", world("hello"));
    let vt = vynt_vec!["x", "y", "z"];
    let ni32 = vynt_vec!(1, 2, 3);
    let nf64 = vynt_vec! {1.2,2.3,5.6};

    println!["{:?}\n{:?}\n{:?}", vt, ni32, nf64];
    let world = S!("World");
    println!("Hello, {}!", world);

    talk!(Vy Dep Trai Nhat Tren Doi Nay Ne);
}
