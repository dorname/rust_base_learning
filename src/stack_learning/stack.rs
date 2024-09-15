#[derive(Debug)]
struct StackItem<T> {
    location:usize,
    data:T
}


#[derive(Debug)]
struct Stack<T> {
    size:usize,
    data:Vec<T>
}

impl<T> Stack<T>{

    /// 这是一个结构体
    /// # Examples:
    /// ```
    /// let mut num:Stack<i32>= Stack::new();
    ///     /// ```
    fn new() -> Self {
        Self {
            size:0,
            data:Vec::new()
        }
    }
    fn push(&mut self, data_item: Option<T>) -> Option<bool> {
        match data_item {
            Some(data) => {
                self.data.push(data);
                self.update_size();
                Some(true)
            }
            None => Some(false),
        }
    }
    fn update_size(&mut self){
        self.size = self.data.len();
    }
    fn len(&self) -> Option<usize>{
        Some(self.size)
    }

    fn first(&self) ->Option<&T>{
        return self.data.first();
    }

    fn insert(&mut self,idx:usize,data_item:T){
        self.data.insert(idx,data_item);
    }

    fn last(&self) ->Option<&T>{
        return self.data.last();
    }
    fn isEmpty(&self) -> Option<bool>{
        Some(self.size == 0)
    }

    fn pop(&mut self) ->Option<bool>{
        self.data.pop();
        self.update_size();
        Some(true)
    }
    fn remove_at(&mut self,idx:usize){
        self.data.remove(idx);
        self.update_size();
    }
    fn query_by_index(&self,idx:usize) -> Option<&T>{
        if self.size >= idx{
            Some(&self.data[idx])
        }else{
            None
        }
        
    }
    fn query(&self) -> Vec<StackItem<&T>>{
        let data_iter = self.data.iter().enumerate();
        let mut vec_stack_items = Vec::new();
        for (index,val) in data_iter {
            let stack_item  = StackItem{
                location:index,
                data:val
            };
            vec_stack_items.push(stack_item);
        }
        vec_stack_items
    }

}

#[test]
fn test() {
    let arr:Vec<i32> = vec![1,2,3];
    let mut num:Stack<i32>= Stack::new();
    num.push(Some(4));
    num.push(Some(5));
    num.push(Some(3));
    println!("序号1的值为{:?}",num.query_by_index(1).unwrap());
    
    println!("结构体{:?}",num);
    num.pop();
    println!("结构体{:?}",num);
    num.remove_at(1);
    println!("结构体{:?}",num);
    num.insert(0,6);
    println!("结构体{:?}",num);

    // println!("{:?}",num.query());
    // if let Some(2) = None{
    //     println!("没有值")
    // }else{
    //     println!("有值")
    // }
}