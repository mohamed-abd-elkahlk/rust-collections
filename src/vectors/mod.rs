///# Vectors (vec![T])
///
/// In programming, a vector typically refers to a dynamic array that can grow or shrink in size. Unlike a fixed-size array, a vector can automatically resize itself to accommodate new elements
///# General Concept:
///
/// * Dynamic Size: Vectors can change their size dynamically as elements are added or removed.
/// * Contiguous Memory Allocation: elements are stored in contiguous memory, allowing for efficient access via indexing.
///
/// * Performance: Inserting or deleting elements at the end of the vector is generally efficient, while operations at the beginning or middle can be slower due to the need to shift elements.
/// # Vector in Rust:
///
/// In Rust, a vector is represented by the Vec<T> type, where T is the type of elements stored in the vector. Some key features of Vec<T> in Rust include and it's Zero indexed :
///
/// * Creation: You can create a vector using the vec! macro, like let vec = vec!\[1, 2, 3\]; , or using Vec::from([1,2,3]) or you create vector using Vec::new().
/// # Examples
/// ```
/// let vec = vec![1, 2, 3]; // 1
/// let vec = let vec = Vec::from([1, 2, 3]); // 2
/// let vec: Vec<i32> = Vec::new(); // 3
/// ```

/// * Push and Pop: You can add elements to the end of a vector using push, and remove the last element using pop.
/// # Examples
/// ## note you should add `mut` keyword before breform any action on the vector
/// ```
/// vec.pop() // removes last elements form the end of vector
/// vec.push() // add elements to the end of a vector
/// ```
/// * Indexing: Elements can be accessed using indexing, like vec[0].
/// # Examples
/// ```
/// vec[0] // will return 1
/// vec[1] // will return 2
/// ```
/// * Safety: Rust ensures that vectors do not go out of bounds, and accessing an invalid index will result in a runtime panic.
/// # Examples
/// ```
/// vec[2] // will return 3
/// vec[3] // this line of code will panic because the vector has onely 3 elements
/// ```

fn vec_opration() {
    let vec: Vec<i32> = Vec::new();
}
