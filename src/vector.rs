use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt;
use std::fmt::Display;


//定义结构体
#[derive(Clone, Copy)]
pub struct Vector3{
    e:[f64; 3]
}

pub type Point3 = Vector3;
pub type Color = Vector3;

impl Vector3{
    pub fn new(e0:f64, e1:f64, e2:f64) -> Vector3{
        Vector3 {
            e:[e0,e1,e2]
        }
    }

    // Vector3 公共方法

    pub fn x(&self) ->f64 {
        self[0]
    }

    pub fn y(&self) ->f64 {
        self[1]
    }

    pub fn z(&self) ->f64 {
        self[2]
    }

    pub fn dot(&self, other: Vector3) ->f64{
        self[0]*other[0]+self[1]*other[1]+self[2]*other[2]
    }

    pub fn length(self) ->f64{
        self.dot(self).sqrt()
    }

    pub fn cross(&self, other: Vector3) ->Vector3{
        Vector3 {
            e:[
                self[1]*other[2] - self[2]*other[1],
                self[2]*other[0] - self[0]*other[2],
                self[0]*other[1] - self[1]*other[0]
            ]
        }
    }

    pub fn normalized(&self) -> Vector3{
        (*self)/self.length()
    }

    // 颜色 公共方法
    pub fn format_color(&self) ->String{
        format!("{} {} {}", (255.99*self[0])as u64, (255.99*self[1])as u64, (255.99*self[2])as u64 )
    }

}

// 函数
impl Index<usize> for Vector3{ //指定类型参数为'usize'
    //创建类型别名
    type Output = f64;

    fn index(&self, index:usize) ->&f64{
        &self.e[index]
    }
}

impl IndexMut<usize> for Vector3{
    fn index_mut(&mut self, index:usize) ->&mut f64{
        &mut self.e[index]
    }
}

impl Add for Vector3{
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3{
        Vector3{
            e:[self[0]+other[0], self[1]+other[1],self[2]+other[2]]
        }
    }
}

impl AddAssign for Vector3{
    fn add_assign(&mut self, other:Vector3)->(){
        *self = Vector3{
            e:[self[0]+other[0], self[1]+other[1],self[2]+other[2]]
        }
    }
}

impl Sub for Vector3{
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3{
        Vector3{
            e:[self[0]-other[0], self[1]-other[1],self[2]-other[2]]
        }
    }
}

impl SubAssign for Vector3{
    fn sub_assign(&mut self, other:Vector3)->(){
        *self = Vector3{
            e:[self[0]-other[0], self[1]-other[1],self[2]-other[2]]
        }
    }
}

impl Mul<f64> for Vector3{
    type Output = Vector3;

    fn mul(self, other:f64) ->Vector3{
        Vector3{
            e:[self[0]*other, self[1]*other, self[2]*other]
        }
    }
}

impl MulAssign<f64> for Vector3{
    fn mul_assign(&mut self, other:f64) ->(){
        *self = Vector3{
            e:[self[0]*other, self[1]*other, self[2]*other]
        }
    }
}

impl Mul<Vector3> for f64{
    type Output = Vector3;

    fn mul(self, other:Vector3) ->Vector3{
        Vector3{
            e:[self*other[0], self*other[1],self*other[2]]
        }
    }
}

impl Div<f64> for Vector3{
    type Output = Vector3;

    fn div(self, other:f64) ->Vector3{
        Vector3{
            e:[self[0]/other, self[1]/other,self[2]/other]
        }
    }
}

impl DivAssign<f64> for Vector3{
    fn div_assign(&mut self, other:f64) -> (){
        *self = Vector3{
            e:[self[0]/other, self[1]/other,self[2]/other]
        }
    }
}

// 查看元素值
impl Display for Vector3{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"({}, {}, {})", self[0], self[1], self[2])
    }
}
