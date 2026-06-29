fn main() 
{
    println!("Hello, world!");
}

mod plant_structures
{
    pub mod roots
    {
        pub mod products
        {
            pub(in crate::plant_structures::roots) struct Cytokinin
            {} //单独指定对哪里public
        }
        use products::Cytokinin;
    }

    pub mod stems
    {
    }

    pub mod leaves
    {
    }
}
