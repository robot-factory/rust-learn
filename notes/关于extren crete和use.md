在很多库的代码里总会看到大段的extern crete， 然而实际教程里又没有相应的介绍，那么extern crete究竟是干什么的呢？

在2018版以前，使用use引用代码里的函数需要事先extern crete， 但是2018版以后， cargo会自动帮我们做相应的事情，所以 extern crete 除了再很少的情况下外基本不再使用了。

那么就有个问题了， 如何通过rustc编译多文件的rs代码呢。