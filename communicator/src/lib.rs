 pub mod network;
 //在顶层父级中进行生命commonclient这个module
 pub mod commonclient;
    fn connect (){
        println!("internet connect u  & I " );
    }

    #[cfg(test)]
    mod tests {
        use super::commonclient;
        #[test]
        fn it_works() {
            commonclient::connect();
        }

    }

    