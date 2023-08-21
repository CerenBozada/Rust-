
struct FilterCondition<T> {
    value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.value
    }
}

fn custom_filter<T>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    collection
        .into_iter()
        .filter(|item| condition.is_match(item))
        .collect()
}

fn main() {
  
    let data = vec![1, 2, 3, 4, 5, 6];


    let condition = FilterCondition { value: 3 };
    let filtered_data = custom_filter(data.clone(), &condition);
    println!("Filtered data: {:?}", filtered_data);
}
