//#[derive(Debug)]
mod stack_heap;
mod pattern_matching;

use std::mem;
use std::ops::Add;

const INITIAL_CONSTANT:u8=32;//no fixed address
//static xyz:i32=123;
static mut xyz:i32=123;
fn datatypes() {
    let a:u8=123; //8bits u:unsigned (whole numbers) 0..255
    println!("a={}",a);
    //a=456;
    // variable immutable println!("a={}",a);
    let mut b:i32=8;
    println!("b={}",b);
    b=456;
    println!("b={}",b);
    let mut c=12345678;//32bit signed integer
    println!("c={}, size={} bytes",c,mem::size_of_val(&c));

    //i8 u8 i16 u16 i32 i64 u64
    c=-422;
    println!("c={} after modification, size={} bytes",c,mem::size_of_val(&c));

    let z:isize=123;//isize /usize
    println!("z={}, size={} bytes,{}-bit OS",z,mem::size_of_val(&z),mem::size_of_val(&z)*8);

    let d='x';
    println!("d={},size of d is {} bytes",d,mem::size_of_val(&d));

    //true or false
    let g=false;
    println!("g={} size is {} bytes",g,mem::size_of_val(&g));

    let h=4>1;
    println!("g={} size is {} bytes",h,mem::size_of_val(&h));

}

fn operators(){
    //arithmetic
    let mut a=2*3+4;
    println!("a={}",a);
    a+=7;
    println!("a={}",a);
    println!("remainder of a/3 is={}",a%3);
    let a_cubed=i32::pow(a,3);
    println!("a cubed={}",a_cubed);
    let b=2.5;
    let b_cubed=f64::powi(b,3);
    let b_power_pi=f64::powf(b,std::f64::consts::PI);
    println!("b cubed is {}, b to the power of pi is {}",b_cubed,b_power_pi);

    //bitwise operators
    let c=1|2;// |OR & AND ^XOR ! NOR
    println!("1|2 ={}",c);

    let two_to_10=1<<10;
    println!("2^10={}",two_to_10);

    //logical operators
    let pi_less_than4=std::f64::consts::PI<4.0;
    let x=5;
    let xis5=x==5;
    println!("{},{}",pi_less_than4,xis5)



}
fn scope_shadowing(){
    let a=123;
    {
        let b=123;
        println!("{}",b);
        let a=77;

        println!("a={}",a);
    }
    println!("{}",a);
}
fn if_statement()
{
    let temp=25;
    if temp>30{
        println!("really hot outside")
    }
    else if temp<10{
        println!("really cold");
    }
    else {
        println!("temp is ok");
    }
    let day=if temp>20{"sunny"} else {"cloudy"};
    println!("today is {}",day);



}

fn while_and_loop(){
    let mut x=1;
    while  x<1000{
        x*=2;
        if x==64 {continue;}
        println!("x={}",x)
    }
    let mut y=1;
    loop {
        y*=2;
        println!("y={}",y);
        if y==1<<10{break;}
    }
}
fn for_loop(){
    for x in 1..11{
        if x==3{continue;}
        println!("{}",x);
    }
    for (pos,y) in (30..41).enumerate(){
        println!("{}:{}",pos,y);
    }

}

fn match_statements(){
    let country_code=777;
    let country=match country_code{
        44=>"uk",
        46=>"sweden",
        7=>"russia",
        //_=>"unknown"
        1...999=>"unknown",
        _=>"invalid"
    };
    println!("the country with code {} id {}",country_code,country);
}
struct Point{
    x:f64,
    y:f64
}
struct Line{
    start:Point,
    end:Point
}

fn structures(){
    let p=Point{x:3.0,y:4.0};
    println!("the point p is at ({},{})",p.x,p.y);
    let p2=Point{x:5.0,y:10.0};
    let myline=Line {start:p,end:p2};
}
enum Color{
    Red,
    Green,
    Blue,
    rgbcolor(u8,u8,u8),
    //cmyk{cyan:u8}//struct
}
fn enums(){
    let c:Color=Color::Red;

    match c {
        Color::Red=>println!("r"),
        Color::Blue=>println!("b"),
        Color::Green=>println!("g"),
        Color::rgbcolor(_,_,_)=>println!("black")
    }

}

union IntorFloat{
    i:i32,
    f:f32
}
fn process_value(iof:IntorFloat){
    unsafe {
        match iof{
            IntorFloat{i:42}=>println!("meaning of life"),
            IntorFloat{f}=>println!("f32={}",f)
        }
    }
}
fn unions(){
    let mut iof=IntorFloat{i:123};

    unsafe{
        iof.i=42;
    }
    let value= unsafe{iof.i};
    process_value(iof);
    process_value(IntorFloat{f:1.33});

}
fn option(){
    //Option<T>
    let x=3.0;
    let y=2.0;
    let result:Option<f64> =
    if y!=0.0 {Some(x/y)} else {None};
    println!("{:?}",result);
    match result{
        Some(z)=>println!("{}/{}={}",x,y,z),
        None=>println!("cannot divide")
    }
    //if let/while let
    if let Some(z)=result{println!("z={}",z);}
}
fn arrays()
{
    let mut a:[i32;5]=[1,2,3,4,5];
    println!("a has {} elements, first is {}",a.len(),a[0]);
    a[0]=321;
    println!("a at o is {}",a[0]);

    println!("{:?}",a);

    if a!=[1,2,3,4,5]{
        println!("doesnot match")
    }

    let b=[1;10];
    for i in 0..b.len(){
        println!("{}",b[i]);
    }
    println!("size of b is {}",mem::size_of_val(&b));

    let mtx:[[f32;3];2]=
    [
        [1.0, 0.0, 0.0],
        [0.0,2.0,0.0]
    ];
    println!("{:?}",mtx);

    for i in 0..mtx.len(){
        
    }

}
fn vectors(){
    let mut a=Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a={:?}",a);
    let idx:usize=0;
    println!("a[0]={}",a[idx]);
    //Option
    match a.get(6){
        Some(x)=>println!("a[6]={}",x),
        None=>println!("error no such a element")
    }

    for x in &a{ println!("{}",x);}
    let last_elem=a.pop();
    while let Some(x)=a.pop(){
        println!("{}",x);
    }



}
fn use_slices(slice: &mut [i32]){
        println!("first element{},len={}",slice[0],slice.len());
        slice[0]=4321;
    }
fn slices()
{
    let mut data=[1,2,3,4,5];
    use_slices(&mut data[1..4]);
    println!("{:?}",data);
}

fn strings(){
    let s="hello there"; //&str= string slice
    for c in s.chars().rev(){
        println!("{}",c);
    }
    if let Some(first_char)=s.chars().nth(0)
    {
        println!("first letter is {}",first_char);
    }

    //heap
    //string
    let mut letters=String::new();
    let mut a='a' as u8;
    while a<=('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a+=1;
    }
    println!("{}",letters);

    //&str <> String
    let u:&str=&letters;
    //concatenation: String +&str
    let z=letters+"abc";
    let mut abc= String::from("hello world");//"hello world".to_string()
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}",abc.replace("ello","goodbye"));

}
fn sum_and_product(x:i32,y:i32)->(i32,i32){
    (x+y,x*y)
}
fn tuples(){
    let x=3;
    let y=4;
    let sp=sum_and_product(x,y);

    println!("{:?}",sp);
    println!("{0}+{1}={2},{0}*{1}={3}",x,y,sp.0,sp.1);

    //destructuring
    let (a,b)=sp;
    println!("{0}+{1}={2},{0}*{1}={3}",x,y,a,b);

    let sp2=sum_and_product(4,7);
    let combined=(sp,sp2);
    println!("{:?}",combined);
    println!("last element={}",(combined.1).1);
    let meaning =(42,);//single elemnt tuple
    println!("{:?}",meaning);

}
struct point<T>{
    x:T,
    y:T
}
struct line<T>
{
    start:point<T>,
    end:point<T>
}
fn generics(){
    let a:point<f64>=point{x:0.0,y:0.0};
    let b=point{x:1.2,y:3.4};
    let myline=line{start:a,end:b};
}

fn print_value(x:i32){
    println!("value={}",x);
}
fn increase(x:&mut i32){
    *x+=1;
}
fn product(x:i32,y:i32)->i32{
    x*y
}
fn functions(){
    print_value(32);
    let mut z=1;
    increase(&mut z);
    println!("z={}",z);
    let a=3;
    let b=4;
    let p=product(a,b);
    println!("{}*{}={}",a,b,p);
}
impl Line{
    fn len(&self)->f64
    {
        let dx=self.start.x-self.end.x;
        let dy=self.start.y-self.end.y;
        (dx*dx+dy*dy).sqrt()

    }
}

fn methods(){
    let p=Point{x:3.0,y:4.0};
    let p2=Point{x:5.0,y:10.0};
    let myline=Line{start:p,end:p2};
    println!("length={}",myline.len());
}
fn say_hello()
{
    println!("hello");
}
fn closures(){
    //say_hello();
    let sh=say_hello;
    sh();
    let plus_one= |x:i32| -> i32 {x+1};
    let a=6;
    println!("{}+1={}",a,plus_one(a));

    let plus_two=|x|{
        let mut z= x;
        z+=2;
        z
    };
    println!("{}+2={}",3,plus_two(3))
}
fn is_even(x:i32)->bool{
    x%2==0
}
fn hof(){
    let limit=500;
    let mut sum=0;
    for i in 0..
    {
        let isq=i*i;
        if isq>limit{break;}
        else if is_even(isq) {sum+=isq;}
    }
    println!("loop sum ={}",sum);
    let sum2=
        (0..).map(|x|x*x)
            .take_while(|&x| x<=limit)
            .filter(|x| is_even(*x))
            .fold(0,|sum,x| sum+x);
    println!("hof sum {}",sum2);
}

trait Animal
{
    fn name(&self)->&'static str;
    fn talk(&self){
        println!("{} cannot talk",self.name());
    }
}
struct Human{
    name:&'static str
}

impl Animal for Human {
    fn name(&self)->&'static str
    {
        self.name
    }
    fn talk(&self){
        println!("{} says hello!",self.name());
    }
}
fn traits(){
let h=Human{name:"John"};
    h.talk()
}
impl Add for Point{
    type Output=Point;
    fn add(self,other:Point)->Point{
        Point{x:self.x+other.x,y:self.y+other.y}
    }
}
fn operator_overloading(){
    let p=Point{x:0.0,y:3.4};
    let p2=Point{x:3.0,y:5.0};
    let p3=p+p2;
    println!("{},{}",p3.x,p3.y);
}
fn main(){

    //operator_overloading()
    //traits()
    //hof();
    //closures();
    //methods();
    //functions()
    //generics()
    //pattern_matching::pattern_matching();
    //tuples();
    //strings()
    //slices()
    //vectors()
    //arrays()
    //option()
    //unions()
    //enums()
    //structures()
    //unsafe{
      //  xyz=777;
        //println!("{}",xyz)
    //stack_heap::stack_and_heap()
    //if_statement();
    //while_and_loop();
    //for_loop()
    //match_statements();
    }

