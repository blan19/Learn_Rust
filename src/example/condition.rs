fn condition() {
  let number = 5;

  if number < 4 {
    println!("condition was true");
  } else {
    println!(condition was false)
  }
}

fn temperature_f(temperature: i32) -> i32 {
  (temperature * 9/5) + 32
}

fn temperature_c(temperature: i32) -> i32 {
  (temperature + 32) * 5/9
}

fn fibo(x: i32) -> i32 {
    if x <= 1 {
        return x;
    }

    return fibo(x - 1) + fibo(x - 2);
}
