use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, SystemTime};

// 1. Shape Trait
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
    fn name(&self) -> &str { "Circle" }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    fn name(&self) -> &str { "Rectangle" }
}

// 2. Drawable Trait
trait Drawable {
    fn draw(&self);
    fn set_color(&mut self, color: &str);
    
    fn render(&self) {
        println!("Rendering...");
        self.draw();
    }
}

struct Button { text: String, color: String }
struct Image { path: String, color: String }

impl Drawable for Button {
    fn draw(&self) { println!("Drawing button: {} ({})", self.text, self.color); }
    fn set_color(&mut self, color: &str) { self.color = color.to_string(); }
}

impl Drawable for Image {
    fn draw(&self) { println!("Drawing image: {} ({})", self.path, self.color); }
    fn set_color(&mut self, color: &str) { self.color = color.to_string(); }
}

// 3. Serializable Trait
trait Serializable {
    fn to_json(&self) -> String;
    fn from_json(json: &str) -> Result<Self, String> where Self: Sized;
    
    fn to_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

#[derive(Debug, Clone)]
struct User { name: String, age: u32 }

impl Serializable for User {
    fn to_json(&self) -> String {
        format!(r#"{{"name": "{}", "age": {}}}"#, self.name, self.age)
    }
    
    fn from_json(_json: &str) -> Result<Self, String> {
        Ok(User { name: "Parsed User".to_string(), age: 25 })
    }
}

// 4. Validator Trait
trait Validator {
    type Error;
    
    fn validate(&self) -> Result<(), Self::Error>;
    
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

struct Email(String);

#[derive(Debug)]
enum EmailError {
    Empty,
    NoAtSymbol,
    InvalidFormat,
}

impl Validator for Email {
    type Error = EmailError;
    
    fn validate(&self) -> Result<(), Self::Error> {
        if self.0.is_empty() {
            return Err(EmailError::Empty);
        }
        if !self.0.contains('@') {
            return Err(EmailError::NoAtSymbol);
        }
        if !self.0.contains('.') {
            return Err(EmailError::InvalidFormat);
        }
        Ok(())
    }
}

// 5. Cache Trait
trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V);
    fn remove(&mut self, key: &K) -> Option<V>;
    fn clear(&mut self);
    
    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

struct MemoryCache<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> MemoryCache<K, V> {
    fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl<K, V> Cache<K, V> for MemoryCache<K, V> 
where 
    K: std::hash::Hash + Eq,
{
    fn get(&self, key: &K) -> Option<&V> { self.data.get(key) }
    fn put(&mut self, key: K, value: V) { self.data.insert(key, value); }
    fn remove(&mut self, key: &K) -> Option<V> { self.data.remove(key) }
    fn clear(&mut self) { self.data.clear(); }
}

// 6. Logger Trait
trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    
    fn info(&self, message: &str) { self.log(LogLevel::Info, message); }
    fn warn(&self, message: &str) { self.log(LogLevel::Warn, message); }
    fn error(&self, message: &str) { self.log(LogLevel::Error, message); }
}

#[derive(Debug)]
enum LogLevel { Info, Warn, Error }

struct ConsoleLogger;
struct FileLogger { path: String }

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{:?}] {}", level, message);
    }
}

impl Logger for FileLogger {
    fn log(&self, level: LogLevel, message: &str) {
        let log_line = format!("[{:?}] {}\n", level, message);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect("Unable to open log file");
        file.write_all(log_line.as_bytes()).expect("Unable to write to log file");
    }
}

// 7. Comparable Trait
trait Comparable<T> {
    fn compare(&self, other: &T) -> std::cmp::Ordering;
    
    fn is_greater_than(&self, other: &T) -> bool {
        matches!(self.compare(other), std::cmp::Ordering::Greater)
    }
    
    fn is_less_than(&self, other: &T) -> bool {
        matches!(self.compare(other), std::cmp::Ordering::Less)
    }
}

struct Student { name: String, grade: f64 }

impl Comparable<Student> for Student {
    fn compare(&self, other: &Student) -> std::cmp::Ordering {
        self.grade.partial_cmp(&other.grade).unwrap_or(std::cmp::Ordering::Equal)
    }
}

// 8. Configurable Trait
trait Configurable {
    fn set_config(&mut self, key: &str, value: String);
    fn get_config(&self, key: &str) -> Option<&String>;
    fn load_from_file(&mut self, path: &str) -> Result<(), String>;
    
    fn get_config_or_default(&self, key: &str, default: &str) -> String {
        self.get_config(key).cloned().unwrap_or_else(|| default.to_string())
    }
}

struct Application {
    config: HashMap<String, String>,
}

impl Application {
    fn new() -> Self {
        Self { config: HashMap::new() }
    }
}

impl Configurable for Application {
    fn set_config(&mut self, key: &str, value: String) {
        self.config.insert(key.to_string(), value);
    }
    
    fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
    
    fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        println!("Loading config from: {}", path);
        self.set_config("debug", "true".to_string());
        self.set_config("port", "8080".to_string());
        Ok(())
    }
}

// 9. Convertible Trait
trait Convertible<T> {
    type Error;
    
    fn convert_to(&self) -> Result<T, Self::Error>;
    fn convert_from(value: T) -> Result<Self, Self::Error> where Self: Sized;
}

struct Celsius(f64);
struct Fahrenheit(f64);

#[derive(Debug)]
struct ConversionError;

impl Convertible<Fahrenheit> for Celsius {
    type Error = ConversionError;
    
    fn convert_to(&self) -> Result<Fahrenheit, Self::Error> {
        Ok(Fahrenheit(self.0 * 9.0 / 5.0 + 32.0))
    }
    
    fn convert_from(f: Fahrenheit) -> Result<Self, Self::Error> {
        Ok(Celsius((f.0 - 32.0) * 5.0 / 9.0))
    }
}

// 10. Processable Trait
trait Processable<T> {
    type Output;
    type Error;
    
    fn process(&self, input: T) -> Result<Self::Output, Self::Error>;
    
    fn process_batch(&self, inputs: Vec<T>) -> Vec<Result<Self::Output, Self::Error>> {
        inputs.into_iter().map(|input| self.process(input)).collect()
    }
}

struct TextProcessor;
struct NumberProcessor;

impl Processable<String> for TextProcessor {
    type Output = String;
    type Error = String;
    
    fn process(&self, input: String) -> Result<Self::Output, Self::Error> {
        if input.is_empty() {
            Err("Empty input".to_string())
        } else {
            Ok(input.to_uppercase())
        }
    }
}

impl Processable<i32> for NumberProcessor {
    type Output = i32;
    type Error = String;
    
    fn process(&self, input: i32) -> Result<Self::Output, Self::Error> {
        if input < 0 {
            Err("Negative number".to_string())
        } else {
            Ok(input * 2)
        }
    }
}

// 11. Queryable Trait
trait Queryable<T> {
    fn find_by_id(&self, id: u32) -> Option<&T>;
    fn find_all(&self) -> Vec<&T>;
    fn filter<F>(&self, predicate: F) -> Vec<&T> where F: Fn(&T) -> bool;
    
    fn count(&self) -> usize {
        self.find_all().len()
    }
}

struct UserRepository {
    users: Vec<User>,
}

impl UserRepository {
    fn new() -> Self {
        Self {
            users: vec![
                User { name: "Alice".to_string(), age: 30 },
                User { name: "Bob".to_string(), age: 25 },
                User { name: "Charlie".to_string(), age: 35 },
            ]
        }
    }
}

impl Queryable<User> for UserRepository {
    fn find_by_id(&self, id: u32) -> Option<&User> {
        self.users.get(id as usize)
    }
    
    fn find_all(&self) -> Vec<&User> {
        self.users.iter().collect()
    }
    
    fn filter<F>(&self, predicate: F) -> Vec<&User> 
    where 
        F: Fn(&User) -> bool 
    {
        self.users.iter().filter(|user| predicate(user)).collect()
    }
}

// 12. Encryptable Trait
trait Encryptable {
    type Key;
    type Error;
    
    fn encrypt(&self, key: &Self::Key) -> Result<Vec<u8>, Self::Error>;
    fn decrypt(data: &[u8], key: &Self::Key) -> Result<Self, Self::Error> where Self: Sized;
    
    fn encrypt_to_string(&self, key: &Self::Key) -> Result<String, Self::Error> {
        self.encrypt(key).map(|bytes| base64_encode(&bytes))
    }
}

fn base64_encode(data: &[u8]) -> String {
    format!("base64_encoded_{}_bytes", data.len())
}

struct Message(String);
struct SimpleKey(String);

#[derive(Debug)]
struct CryptoError;

impl Encryptable for Message {
    type Key = SimpleKey;
    type Error = CryptoError;
    
    fn encrypt(&self, key: &Self::Key) -> Result<Vec<u8>, Self::Error> {
        let mut result = self.0.as_bytes().to_vec();
        let key_byte = key.0.as_bytes().get(0).unwrap_or(&0);
        for byte in &mut result {
            *byte ^= key_byte;
        }
        Ok(result)
    }
    
    fn decrypt(data: &[u8], key: &Self::Key) -> Result<Self, Self::Error> {
        let mut result = data.to_vec();
        let key_byte = key.0.as_bytes().get(0).unwrap_or(&0);
        for byte in &mut result {
            *byte ^= key_byte;
        }
        Ok(Message(String::from_utf8_lossy(&result).to_string()))
    }
}

// 13. Observable Trait (Simplified for demo)
trait Observable<T> {
    fn notify(&self, data: &T);
}

struct EventEmitter<T> {
    name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> EventEmitter<T> {
    fn new(name: &str) -> Self {
        Self { 
            name: name.to_string(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: std::fmt::Debug> Observable<T> for EventEmitter<T> {
    fn notify(&self, data: &T) {
        println!("EventEmitter '{}' notifying: {:?}", self.name, data);
    }
}

// 14. Buildable Trait
trait Buildable {
    type Output;
    
    fn build(self) -> Self::Output;
    fn reset(&mut self);
}

#[derive(Clone)]
struct CarBuilder {
    make: Option<String>,
    model: Option<String>,
    year: Option<u32>,
}

struct Car {
    make: String,
    model: String,
    year: u32,
}

impl CarBuilder {
    fn new() -> Self {
        Self { make: None, model: None, year: None }
    }
    
    fn make(mut self, make: &str) -> Self {
        self.make = Some(make.to_string());
        self
    }
    
    fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }
    
    fn year(mut self, year: u32) -> Self {
        self.year = Some(year);
        self
    }
}

impl Buildable for CarBuilder {
    type Output = Result<Car, String>;
    
    fn build(self) -> Self::Output {
        Ok(Car {
            make: self.make.ok_or("Make is required")?,
            model: self.model.ok_or("Model is required")?,
            year: self.year.ok_or("Year is required")?,
        })
    }
    
    fn reset(&mut self) {
        self.make = None;
        self.model = None;
        self.year = None;
    }
}

// 15. Schedulable Trait
trait Schedulable {
    fn schedule(&self, delay: Duration);
    fn schedule_at(&self, time: SystemTime);
    fn cancel(&self);
    fn is_scheduled(&self) -> bool;
    
    fn schedule_repeating(&self, interval: Duration) {
        println!("Scheduling repeating task every {:?}", interval);
    }
}

struct Task {
    id: u32,
    name: String,
    scheduled: bool,
}

impl Schedulable for Task {
    fn schedule(&self, delay: Duration) {
        println!("Task '{}' (ID: {}) scheduled to run in {:?}", self.name, self.id, delay);
    }
    
    fn schedule_at(&self, time: SystemTime) {
        println!("Task '{}' (ID: {}) scheduled to run at {:?}", self.name, self.id, time);
    }
    
    fn cancel(&self) {
        println!("Task '{}' (ID: {}) cancelled", self.name, self.id);
    }
    
    fn is_scheduled(&self) -> bool {
        self.scheduled
    }
}

// MAIN FUNCTION - Demonstrates all traits
fn main() {
    println!("🦀 Rust Traits Demo - 15 Examples\n");
    println!("{}", "=".repeat(50));
    
    // 1. Shape Trait Demo
    println!("\n1. 📐 SHAPE TRAIT");
    println!("{}", "-".repeat(20));
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };
    
    println!("{}: Area = {:.2}, Perimeter = {:.2}", 
             circle.name(), circle.area(), circle.perimeter());
    println!("{}: Area = {:.2}, Perimeter = {:.2}", 
             rectangle.name(), rectangle.area(), rectangle.perimeter());
    
    // 2. Drawable Trait Demo
    println!("\n2. 🎨 DRAWABLE TRAIT");
    println!("{}", "-".repeat(20));
    let mut button = Button { 
        text: "Click Me".to_string(), 
        color: "blue".to_string() 
    };
    let mut image = Image { 
        path: "/path/to/image.png".to_string(), 
        color: "transparent".to_string() 
    };
    
    button.render();
    button.set_color("red");
    button.draw();
    
    image.render();
    image.set_color("sepia");
    image.draw();
    
    // 3. Serializable Trait Demo
    println!("\n3. 📄 SERIALIZABLE TRAIT");
    println!("{}", "-".repeat(20));
    let user = User { name: "John Doe".to_string(), age: 30 };
    println!("User JSON: {}", user.to_json());
    println!("User bytes length: {}", user.to_bytes().len());
    
    match User::from_json(r#"{"name": "Jane", "age": 25}"#) {
        Ok(parsed_user) => println!("Parsed user: {:?}", parsed_user),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // 4. Validator Trait Demo
    println!("\n4. ✅ VALIDATOR TRAIT");
    println!("{}", "-".repeat(20));
    let valid_email = Email("user@example.com".to_string());
    let invalid_email = Email("invalid-email".to_string());
    let empty_email = Email("".to_string());
    
    println!("user@example.com is valid: {}", valid_email.is_valid());
    println!("invalid-email is valid: {}", invalid_email.is_valid());
    println!("empty email is valid: {}", empty_email.is_valid());
    
    if let Err(e) = invalid_email.validate() {
        println!("Validation error: {:?}", e);
    }
    
    // 5. Cache Trait Demo
    println!("\n5. 💾 CACHE TRAIT");
    println!("{}", "-".repeat(20));
    let mut cache: MemoryCache<String, String> = MemoryCache::new();
    
    cache.put("user:1".to_string(), "John Doe".to_string());
    cache.put("user:2".to_string(), "Jane Smith".to_string());
    
    println!("Cache contains user:1: {}", cache.contains_key(&"user:1".to_string()));
    println!("Get user:1: {:?}", cache.get(&"user:1".to_string()));
    println!("Get user:3: {:?}", cache.get(&"user:3".to_string()));
    
    cache.remove(&"user:1".to_string());
    println!("After removal, user:1: {:?}", cache.get(&"user:1".to_string()));
    
    // 6. Logger Trait Demo
    println!("\n6. 📝 LOGGER TRAIT");
    println!("{}", "-".repeat(20));
    let console_logger = ConsoleLogger;
    let file_logger = FileLogger { path: "app.log".to_string() };
    
    console_logger.info("Application started");
    console_logger.warn("Low memory warning");
    console_logger.error("Database connection failed");
    
    file_logger.info("File log entry");
    file_logger.error("Critical error logged to file");
    
    // 7. Comparable Trait Demo
    println!("\n7. ⚖️ COMPARABLE TRAIT");
    println!("{}", "-".repeat(20));
    let student1 = Student { name: "Alice".to_string(), grade: 85.5 };
    let student2 = Student { name: "Bob".to_string(), grade: 92.0 };
    let student3 = Student { name: "Charlie".to_string(), grade: 78.0 };
    
    println!("{} > {}: {}", student1.name, student2.name, student1.is_greater_than(&student2));
    println!("{} > {}: {}", student2.name, student3.name, student2.is_greater_than(&student3));
    println!("{} < {}: {}", student3.name, student1.name, student3.is_less_than(&student1));
    
    // 8. Configurable Trait Demo
    println!("\n8. ⚙️ CONFIGURABLE TRAIT");
    println!("{}", "-".repeat(20));
    let mut app = Application::new();
    
    app.set_config("app_name", "MyApp".to_string());
    app.set_config("version", "1.0.0".to_string());
    
    println!("App name: {}", app.get_config_or_default("app_name", "Unknown"));
    println!("Port: {}", app.get_config_or_default("port", "3000"));
    
    if let Ok(_) = app.load_from_file("config.json") {
        println!("Config loaded successfully");
        println!("Debug mode: {}", app.get_config_or_default("debug", "false"));
        println!("Port after load: {}", app.get_config_or_default("port", "3000"));
    }
    
    // 9. Convertible Trait Demo
    println!("\n9. 🔄 CONVERTIBLE TRAIT");
    println!("{}", "-".repeat(20));
    let celsius = Celsius(25.0);
    let fahrenheit = Fahrenheit(77.0);
    
    match celsius.convert_to() {
        Ok(f) => println!("25°C = {:.1}°F", f.0),
        Err(_) => println!("Conversion failed"),
    }
    
    match Celsius::convert_from(fahrenheit) {
        Ok(c) => println!("77°F = {:.1}°C", c.0),
        Err(_) => println!("Conversion failed"),
    }
    
    // 10. Processable Trait Demo
    println!("\n10. ⚡ PROCESSABLE TRAIT");
    println!("{}", "-".repeat(20));
    let text_processor = TextProcessor;
    let number_processor = NumberProcessor;
    
    let texts = vec!["hello".to_string(), "world".to_string(), "".to_string()];
    let numbers = vec![5, -3, 10, 0];
    
    println!("Text processing results:");
    for result in text_processor.process_batch(texts) {
        match result {
            Ok(processed) => println!("  ✓ {}", processed),
            Err(e) => println!("  ✗ Error: {}", e),
        }
    }
    
    println!("Number processing results:");
    for result in number_processor.process_batch(numbers) {
        match result {
            Ok(processed) => println!("  ✓ {}", processed),
            Err(e) => println!("  ✗ Error: {}", e),
        }
    }
    
    // 11. Queryable Trait Demo
    println!("\n11. 🔍 QUERYABLE TRAIT");
    println!("{}", "-".repeat(20));
    let user_repo = UserRepository::new();
    
    println!("Total users: {}", user_repo.count());
    println!("All users:");
    for user in user_repo.find_all() {
        println!("  - {} (age: {})", user.name, user.age);
    }
    
    println!("Users over 30:");
    for user in user_repo.filter(|u| u.age > 30) {
        println!("  - {} (age: {})", user.name, user.age);
    }
    
    if let Some(user) = user_repo.find_by_id(1) {
        println!("User at index 1: {} (age: {})", user.name, user.age);
    }
    
    // 12. Encryptable Trait Demo
    println!("\n12. 🔐 ENCRYPTABLE TRAIT");
    println!("{}", "-".repeat(20));
    let message = Message("Secret Message".to_string());
    let key = SimpleKey("mykey".to_string());
    
    match message.encrypt(&key) {
        Ok(encrypted) => {
            println!("Original: {}", message.0);
            println!("Encrypted bytes: {:?}", encrypted);
            println!("Encrypted string: {}", message.encrypt_to_string(&key).unwrap());
            
            match Message::decrypt(&encrypted, &key) {
                Ok(decrypted) => println!("Decrypted: {}", decrypted.0),
                Err(_) => println!("Decryption failed"),
            }
        },
        Err(_) => println!("Encryption failed"),
    }
    
    // 13. Observable Trait Demo
    println!("\n13. 👁️ OBSERVABLE TRAIT");
    println!("{}", "-".repeat(20));
    let user_events = EventEmitter::<&'static str>::new("UserEvents");
    let system_events = EventEmitter::<i32>::new("SystemEvents");
    
    user_events.notify(&"User logged in");
    user_events.notify(&"User updated profile");
    system_events.notify(&42);
    // system_events.notify(&"System maintenance scheduled"); // This would be a type error
    
    // 14. Buildable Trait Demo
    println!("\n14. 🏗️ BUILDABLE TRAIT");
    println!("{}", "-".repeat(20));
    let car_result = CarBuilder::new()
        .make("Toyota")
        .model("Camry")
        .year(2023)
        .build();
    
    match car_result {
        Ok(car) => println!("Built car: {} {} {}", car.year, car.make, car.model),
        Err(e) => println!("Build failed: {}", e),
    }
    
    // Try building without required fields
    let incomplete_car = CarBuilder::new()
        .make("Honda")
        .build();
    
    match incomplete_car {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected build failure: {}", e),
    }
    
    // 15. Schedulable Trait Demo
    println!("\n15. ⏰ SCHEDULABLE TRAIT");
    println!("{}", "-".repeat(20));
    let task1 = Task { id: 1, name: "Backup Database".to_string(), scheduled: false };
    let task2 = Task { id: 2, name: "Send Emails".to_string(), scheduled: true };
    
    task1.schedule(Duration::from_secs(300)); // 5 minutes
    task2.schedule_at(SystemTime::now());
    task1.schedule_repeating(Duration::from_secs(3600)); // 1 hour
    
    println!("Task 1 scheduled: {}", task1.is_scheduled());
    println!("Task 2 scheduled: {}", task2.is_scheduled());
    
    task2.cancel();
    
    println!("\n🎉 All trait examples completed successfully!");
    println!("{}", "=".repeat(50));
}
