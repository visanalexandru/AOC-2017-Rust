use std::fs::File;
use std::io::{BufReader,BufRead};
use std::collections::HashMap;

fn get_lines(filename:&str) -> Vec<String>{
    let file=match File::open(filename){
        Ok(file) => file,
        Err(err) => panic!("Cannot open file: {}",err)
    };
    let reader=BufReader::new(file);
    let lines=reader.lines();
    let mut result=Vec::new();

    for line in lines{
        let data=match line{
            Ok(data) => data,
            Err(err) => panic!("Cannot read line: {}",err)
        };
        result.push(data);
    }

    result
}

fn get_numbers(lines:&Vec<String>) -> Vec<u32>{
    let tokens=lines[0].split_whitespace();
    let mut result=Vec::new();
    for token in tokens{
        let number:u32=token.parse().unwrap();
        result.push(number);
    }
    result
}

//Returns the index of the maximum element in the array
fn get_maximum(nums:&Vec<u32>) -> usize{
    let mut result=0;

    for (index,&number) in nums.iter().enumerate(){
        if number>nums[result]{
            result=index;
        }
    } 

    result
}

fn solve(nums:&Vec<u32>) -> (u32,u32){
    let mut nums=nums.clone();
    let size=nums.len();
    let mut cycle=0;
    let mut seen=HashMap::new();

    loop{
        let mut current=get_maximum(&nums);
        let value=nums[current];
        nums[current]=0;
        
        for _ in 0..value{
            current=(current+1)%size;
            nums[current]+=1;
        }

        cycle+=1; 

        if seen.contains_key(&nums){
            break;
        }

        seen.insert(nums.clone(),cycle);

    }

    let old:u32=*seen.get(&nums).unwrap();

    (cycle,cycle-old)
}

fn main() {
    let lines=get_lines("input");
    let nums=get_numbers(&lines);
    let result=solve(&nums);
    println!("{}",result.0);
    println!("{}",result.1);
}
