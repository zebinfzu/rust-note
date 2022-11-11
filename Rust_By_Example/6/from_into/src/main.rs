fn main() {
    // From 和 Into 两个特征内部关联，实现From特征就实现Into
    {
        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        let num = Number::from(30);
        println!("My number is {:?}", num);

        // 同时也自动实现into
        let int = 5;
        // 试试删除类型说明
        let num: Number = int.into();
        println!("My number is {:?}", num);
    }
    // 类似于From和Into，TryFrom和TryInto用于容易出错的转换，因此返回值是Result类型
    {
        #[derive(Debug, PartialEq)]
        struct EvenNumber(i32);

        impl TryFrom<i32> for EvenNumber {
            type Error = ();
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNumber(value))
                } else {
                    Err(())
                }
            }
        }

        // TryFrom
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        // TryInto
        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    }
}
