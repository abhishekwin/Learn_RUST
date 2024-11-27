#[derive(Clone, Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn print_username(&self) {
        println!("{}", self.username);
    }

    // field shadowing
    fn active(&self) -> bool {
        self.active
    }

    // Executing this method will put the current object from out of scope
    fn reset_count(mut self) -> User {
        self.sign_in_count = 0;
        self
    }

    fn is_same_user(&self, next: &User) -> bool {
        self.username == next.username && self.email == next.email
    }
    fn create_test(email: String) -> Self {
        Self {
            active: true,
            username: String::from("test"),
            email: email,
            sign_in_count: 0,
        }
    }
}

// Tuple Strcts
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit like Struct
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    // Create struct with all values
    let user = User {
        active: true,
        username: String::from("Abhishek"),
        email: String::from("Test1@example.com"),
        sign_in_count: 1,
    };
    println!("\nUser1: {:#?}", user);

    // Cloning a value from previous struct and replacing it
    let mut user2 = user.clone();
    user2.email = String::from("Test2@example.com");
    println!("\nUser2: {:#?}", user2);

    // Creating a new struct by giving some values and some by default
    let mut user3 = create_new_user(String::from("Demo@example.com"), String::from("user3"));
    user3.email = String::from("Test3@example.com");
    println!("\nUser3: {:#?}", user3);

    // Create struct with shorthand syntax
    let user4 = create_user_shorthand(String::from("Demo@example.com"), String::from("user4"));
    println!("\nUser4: {:#?}", user4);

    // Creating a struct from cloning user4 and setting active to false
    let user5 = User {
        active: false,
        ..user4.clone()
    };
    println!("\nUser5: {:#?}", user5);

    // Creating new object with few details and from other structs with transferring details
    let user6 = User {
        sign_in_count: 0,
        ..user3 // After this point user3 won't have email and user name as it's values has been borrowed by user5
    };
    println!("\nUser6: {:#?}", user6);

    let user7 = User {
        email: String::from("email@test.com"),
        username: String::from("test"),
        ..user5 // user5 will still work as the scaler type values are copied(ie. bool and u65)
    };
    println!("\nuser7: {:#?}", user6);
    println!("\nuser5: {:#?}", user5);

    user7.print_username();
    // Using method
    if !user7.active() {
        println!("\nuser7 is inactive.")
    }
    // Using property
    println!("User's7  active status is: {}", user7.active);

    if user6.is_same_user(&user7) {
        println!("User6 and user7 are same");
    } else {
        println!("User6 and user7 are not same");
    }

    let new_user7 = user7.reset_count();
    println!("\nnew_user7: {:#?}", new_user7);
    // println!("\nuser7: {:#?}", user7); // This will throw error as after calling user7.reset_count(), user7 is out of scope

    // Associated functions
    let user8 = User::create_test(String::from("test@mail.com"));
    println!("\nuser8: {:#?}", user8);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:#?}", black);
    println!("origin: {:#?}", origin);

    let subject = AlwaysEqual;
    println!("subject: {:#?}", subject);
}

fn create_new_user(email: String, username: String) -> User {
    let user = User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    };
    user
}

fn create_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
