mod client;

/*
trait  Interface  {
   fn get_name<'a> ()  -> &'a str    ;
   fn get_version<'a >( ) -> &'a str ; fn

} */

trait Interface  {
    const NAME: String;
    const VERSION : String ;



}


trait Message {

}





struct  Display {
    #[cfg(feature = "client")]
    a : String


}




#[cfg(feature = "client")]
impl Display {

}
#[cfg(feature = "client")]
impl Display{

}





pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
