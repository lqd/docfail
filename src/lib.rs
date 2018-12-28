pub trait Leaper {
}

pub struct FilterAnti<Func>
{
    phantom: ::std::marker::PhantomData<Func>,
}

// crashes rustdoc
impl<Func: Fn() -> (u32)>
    Leaper for FilterAnti<Func>
where
    Func: Fn() -> (u32),
{
}

// works
// impl<Func> Leaper for FilterAnti<Func>
// where
//     Func: Fn() -> (u32),
// {
// }

// works
// impl<Func: Fn() -> (u32)>
//     Leaper for FilterAnti<Func>
// {
// }