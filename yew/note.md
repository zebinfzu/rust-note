# yew

## install

1.  wasm 依赖

    ```shell
    rustup target add wasm32-unknown-unknown
    ```

2.  trunk，类似 create-react-app 这样的 cli 工具

    ```shell
    cargo install trunk
    ```

## hello world

1. yew 框架提供的 html!宏可以返回 Html 元素
2. 这个宏当中只能有一个根元素
3. 标签的写法和常规的 HTML 没什么区别，但是必须要闭合
4. 标签的内容 rust 是不认识的，必须使用{}并且插入 rust 的数据格式，比如&str
5. 同时一个返回 Html 的函数并不能作为一个组件，必须给这个函数加上#\[function_component(组件名)\]的宏

   ```rust
   #[function_component(App)]
   fn app() -> Html {
     html! {
       <h1>{"Hello world"}</h1>
     }
   }
   ```

6. 主函数渲染组件

   ```rust
   fn main() {
     yew::Renderer::<App>::new().render();
   }
   ```

7. 项目文件夹添加 index.html
8. 启动项目 trunk serve

## 调试

1. 由于代码编译完是 wasm 文件运行在浏览器里面，所以 println 宏用不了
2. 第三方的 crate gloo
3. gloo 提供了许多 console.log 之类的方法，可以在浏览器的命令行打印
4. log!宏使用方式很像 console.log，但是不支持打印 structure，会警告要转换成 JsObject
5. 或者可以用序列化的库，serde 和 serde_json

```rs
#[derive(Serialize, Deserialize)]
struct MyObject {
  username: String,
  favorite_language: String
}
// 跳过定义my_object
log!(serde_json::to_string_pretty(&my_object).unwrap());
```

## html in yew

1. 语法和 jsx 类似
2. 可以直接使用 class，rust 没有这个关键字
   ```rs
   let class = "my_title";
   html! {
       <h1 class="title">{"Hello world"}</h1>
       <h1 class={class}>{"Good bey!"}</h1>
   }
   ```
3. 上面两个根标签是会报错的，需要和 react 一样使用<></>来作为假根标签，同样渲染的 html 文件不会有假根标签

### 条件渲染

总之用法和 rust 本来的条件语句差不多，这些空值语句会让不被渲染的东西不产生 DOM 节点

1. if、if-else

   ```rs
   html! {
   if true {
       <p>{ "True case" }</p>
   }
   };
   let some_condition = true;

   html! {
    if some_condition {
        <p>{ "True case" }</p>
    } else {
        <p>{ "False case" }</p>
    }
   };
   ```

2. if-let、if-let-else

   ```rs
   let some_text = Some("text");

   html! {
       if let Some(text) = some_text {
           <p>{ text }</p>
       }
   };

   let some_text = Some("text");

    html! {
        if let Some(text) = some_text {
            <p>{ text }</p>
        } else {
            <p>{ "False case" }</p>
        }
    };
   ```

### html loop

1. html!宏里面能显示的数据类型还是 Html，yew 也可以展示 Vec<Html>
2. 因此可以将需要的展示的数据通过迭代器的方式转换成 html!节点再收集起来
3. 避免过多复杂逻辑耦合，记得拆分成函数

```rs
let tasks = vec!["record video", "grocery shopping", "pet Xilbe"];
<ul>
  {tasks.iter().map(|t| html! {<li>{t}</li>}).collect::<Html>()}
</ul>
```

## css

### stylist

1. crate stylist,需要导入特性 yew
2. 将组件原来的 function_component 改成 styled_component
3. 可以使用 css!宏和 style!宏添加样式了
4. style!()宏内部给字符串，就是正常css写法，可以使用rust中多行无转义字符串
5. css!()宏内部也相同，不过一般用在行内样式
6. 注意stylesheet可以定义区块内所有的样式，直接给加在根元素上就好了，里面按照正常选择器去写就行
7. 同样可以做局部样式表，使用include_str!()宏直接导入css文件，然后传给Style::new()

```rs
let stylesheet = style!(
      r#"
        color: red;
      "#
    );
html! {
  <div class={stylesheet.unwrap()}>{"red text"}</div>
}
html! {
  <div class={css!("color: red; font-size: 75px;")}>{"red text"}</div>
}
```

## component

1. src 下新建 components 文件夹管理组件
2. atoms(分子)用来丢一些不可以再分的小组件，比如按钮之类的
3. molecules(原子)用来丢一些组合起来的小组件
4. organisms(有机物)用来丢一些比如状态栏，侧边栏之类复杂的组件
5. pages 用来放整个页面
