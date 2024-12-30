#[cxx::bridge(namespace = "example")]
mod ffi {
    #[namespace = "example_enum"]
    enum ExampleEnum {
        A,
        B,
    }

    struct ExampleStruct {
        example: ExampleEnum,
    }

    unsafe extern "C++" {
        include!("lib.h");
        type ExampleEnum;

        fn create_example_struct() -> UniquePtr<ExampleStruct>;
    }
}


#[cfg(test)]
mod tests {
    use crate::ffi::create_example_struct;

    #[test]
    fn it_works() {
        let _result = create_example_struct();
    }
}
