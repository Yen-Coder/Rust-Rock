// Define the User struct first
#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
}

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
    category: String,
}

// 1. Basic Queryable Trait
trait Queryable<T> {
    fn find_by_id(&self, id: u32) -> Option<&T>;
    fn find_all(&self) -> Vec<&T>;
    fn filter<F>(&self, predicate: F) -> Vec<&T> where F: Fn(&T) -> bool;
    
    fn count(&self) -> usize {
        self.find_all().len()
    }
}

// 2. Basic UserRepository
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

// 3. ProductRepository
struct ProductRepository {
    products: Vec<Product>,
}

impl ProductRepository {
    fn new() -> Self {
        Self {
            products: vec![
                Product { name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string() },
                Product { name: "Book".to_string(), price: 19.99, category: "Education".to_string() },
                Product { name: "Phone".to_string(), price: 699.99, category: "Electronics".to_string() },
                Product { name: "Pen".to_string(), price: 2.99, category: "Office".to_string() },
            ]
        }
    }
}

impl Queryable<Product> for ProductRepository {
    fn find_by_id(&self, id: u32) -> Option<&Product> {
        self.products.get(id as usize)
    }
    
    fn find_all(&self) -> Vec<&Product> {
        self.products.iter().collect()
    }
    
    fn filter<F>(&self, predicate: F) -> Vec<&Product> 
    where 
        F: Fn(&Product) -> bool 
    {
        self.products.iter().filter(|product| predicate(product)).collect()
    }
    
    // Override count for better performance
    fn count(&self) -> usize {
        self.products.len()
    }
}

// 4. Extension trait for more complex queries
trait QueryableExt<T>: Queryable<T> {
    fn find_first<F>(&self, predicate: F) -> Option<&T>
    where 
        F: Fn(&T) -> bool,
    {
        self.filter(predicate).into_iter().next()
    }
    
    fn exists<F>(&self, predicate: F) -> bool
    where 
        F: Fn(&T) -> bool,
    {
        self.find_first(predicate).is_some()
    }
    
    fn count_where<F>(&self, predicate: F) -> usize
    where 
        F: Fn(&T) -> bool,
    {
        self.filter(predicate).len()
    }
}

// Blanket implementation for all Queryable types
impl<T, Q: Queryable<T>> QueryableExt<T> for Q {}

// 5. Advanced UserRepository with CRUD operations
struct AdvancedUserRepository {
    users: Vec<User>,
    next_id: u32,
}

impl AdvancedUserRepository {
    fn new() -> Self {
        Self {
            users: vec![
                User { name: "Alice".to_string(), age: 30 },
                User { name: "Bob".to_string(), age: 25 },
                User { name: "Charlie".to_string(), age: 35 },
            ],
            next_id: 3,
        }
    }
    
    fn insert(&mut self, user: User) -> u32 {
        let id = self.next_id;
        self.users.push(user);
        self.next_id += 1;
        id
    }
    
    fn update<F>(&mut self, id: u32, updater: F) -> bool
    where 
        F: FnOnce(&mut User),
    {
        if let Some(user) = self.users.get_mut(id as usize) {
            updater(user);
            true
        } else {
            false
        }
    }
    
    fn delete(&mut self, id: u32) -> bool {
        if (id as usize) < self.users.len() {
            self.users.remove(id as usize);
            true
        } else {
            false
        }
    }
}

impl Queryable<User> for AdvancedUserRepository {
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
    
    fn count(&self) -> usize {
        self.users.len()
    }
}

// 6. Generic helper functions
fn print_all_items<T, Q>(repository: &Q, item_name: &str) 
where 
    Q: Queryable<T>,
    T: std::fmt::Debug,
{
    println!("All {}s:", item_name);
    for item in repository.find_all() {
        println!("  {:?}", item);
    }
    println!("Total count: {}", repository.count());
}

fn find_and_print<T, Q, F>(repository: &Q, predicate: F, description: &str)
where 
    Q: Queryable<T>,
    T: std::fmt::Debug,
    F: Fn(&T) -> bool,
{
    let results = repository.filter(predicate);
    println!("{}: {} items found", description, results.len());
    for item in results {
        println!("  {:?}", item);
    }
}

// 7. Demonstration functions
fn demonstrate_basic_queries() {
    println!("=== Basic Queryable Trait Demo ===");
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
    println!();
}

fn demonstrate_product_queries() {
    println!("=== Product Repository Demo ===");
    let product_repo = ProductRepository::new();
    
    // Find expensive products
    let expensive_products = product_repo.filter(|p| p.price > 500.0);
    println!("Expensive products:");
    for product in expensive_products {
        println!("  - {} (${:.2})", product.name, product.price);
    }
    
    // Find electronics
    let electronics = product_repo.filter(|p| p.category == "Electronics");
    println!("Electronics: {} items", electronics.len());
    for product in electronics {
        println!("  - {} (${:.2})", product.name, product.price);
    }
    println!();
}

fn demonstrate_generic_queries() {
    println!("=== Generic Query Functions Demo ===");
    let user_repo = UserRepository::new();
    let product_repo = ProductRepository::new();
    
    // Works with any Queryable type
    print_all_items(&user_repo, "user");
    println!();
    print_all_items(&product_repo, "product");
    println!();
    
    // Generic filtering
    find_and_print(&user_repo, |u| u.age > 30, "Users over 30");
    println!();
    find_and_print(&product_repo, |p| p.price < 50.0, "Cheap products");
    println!();
}

fn demonstrate_extended_queries() {
    println!("=== Extended Queries Demo ===");
    let user_repo = UserRepository::new();
    
    // Find first user over 30
    if let Some(user) = user_repo.find_first(|u| u.age > 30) {
        println!("First user over 30: {}", user.name);
    }
    
    // Check if any user is named "Alice"
    if user_repo.exists(|u| u.name == "Alice") {
        println!("Alice exists in the repository");
    }
    
    // Count users in different age ranges
    let young_count = user_repo.count_where(|u| u.age < 30);
    let old_count = user_repo.count_where(|u| u.age >= 30);
    
    println!("Young users: {}, Older users: {}", young_count, old_count);
    println!();
}

fn demonstrate_crud_operations() {
    println!("=== CRUD Operations Demo ===");
    let mut repo = AdvancedUserRepository::new();
    
    // Create
    let new_id = repo.insert(User { name: "David".to_string(), age: 28 });
    println!("Inserted user with ID: {}", new_id);
    
    // Read
    if let Some(user) = repo.find_by_id(new_id) {
        println!("Found user: {} (age: {})", user.name, user.age);
    }
    
    // Update
    let updated = repo.update(new_id, |user| {
        user.age += 1;
        println!("Updated {}'s age to {}", user.name, user.age);
    });
    println!("Update successful: {}", updated);
    
    // Query after update
    if let Some(user) = repo.find_by_id(new_id) {
        println!("User after update: {} (age: {})", user.name, user.age);
    }
    
    // Query all adults
    let adults = repo.filter(|u| u.age >= 18);
    println!("Adult users: {}", adults.len());
    
    // Show all users
    println!("All users in advanced repository:");
    for (i, user) in repo.find_all().iter().enumerate() {
        println!("  [{}] {} (age: {})", i, user.name, user.age);
    }
    
    println!();
}

fn main() {
    demonstrate_basic_queries();
    demonstrate_product_queries();
    demonstrate_generic_queries();
    demonstrate_extended_queries();
    demonstrate_crud_operations();
    
    println!("=== Summary ===");
    println!("The Queryable trait demonstrates:");
    println!("1. Generic trait design for reusable query interfaces");
    println!("2. Default implementations with override capability");
    println!("3. Closure-based filtering for flexible queries");
    println!("4. Extension traits for additional functionality");
    println!("5. CRUD operations building on the query foundation");
    println!("6. Zero-cost abstractions with compile-time optimisation");
}
