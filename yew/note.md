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
7. 项目文件夹添加index.html
8. 启动项目trunk serve