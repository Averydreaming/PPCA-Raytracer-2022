
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
pub struct onb{
    pub axis:[Vec3;3],
}
impl onb{
    fn u(&self)->Vec3{
        return self.axis[0];
    }
    fn v(&self)->Vec3{
        return self.axis[1];
    }
    fn w(&self)->Vec3{
        return self.axis[2];
    }
    fn local(&self,a:f64,b:f64,c:f64)->Vec3{
        return self.u()*a+self.v()*b+self.w()*c;
    }
    fn local_vec(a:Vec3)->Vec3{
        return self.u()*a.x+self.v()*x.y+self.w()*a.z;
    }
    fn build_from_w(n:Vec3)->Self{
        let w=Vec3::unit_vector(n);
        if (w.x.abs()>0.9) { let a=Vec3::new(0.,1.,0.);} else { let a=Vec3::new(1.,0.,0.);} 
        let v=Vec3::unit_vector(Vec3::cross(w,a));
        let u=Vec3::cross(w, v);
        Self{
            axis:[u,v,w]
        }
    }
}

impl Index<usize> for onb {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.axis[0],
            1 => &self.axis[1],
            2 => &self.axis[2],
            _ => panic!("Try to get {}th dimension of onb.", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.axis[0],
            1 => &mut self.axis[1],
            2 => &mut self.axis[2],
            _ => panic!("Try to get {}th dimension of onb.", index),
        }
    }
}
