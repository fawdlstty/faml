# 前言

提到faml，不得不说toml。toml是一款简单清晰的配置语言，结合了多种配置语言的优点，便于理解和编写。它支持注释、嵌套结构，语法简洁明了，特别适合用作配置文件，比JSON和YAML更具可读性，减少了手动编辑时出错的可能性。

作为一款静态配置语言，toml也许太成功了，它成功的让很多有动态配置需求的场合用上了它。比如rust项目管理文件：Cargo.toml。下面以 `rust-jemalloc-pprof` 项目引用代码为例：

```toml
[dependencies]
[target.'cfg(not(target_env = "msvc"))'.dependencies]
tikv-jemallocator = { version = "0.6.0", features = ["profiling", "unprefixed_malloc_on_supported_platforms"] }
```

简单来说就是，这是一个动态配置需求，需求是target_env为msvc时不生效，其他情况下生效。实际处理时会吧单引号引起来的内容`cfg(not(target_env = "msvc"))`当做一个字符串来解析，然后将其作为另一种语法的语言来执行。此处会带来两个问题：
1. 不同语言语法混用使得编写复杂，带很多hack
2. 嵌入的另一种语言没有明确的语法规范，其他程序完全可以再嵌入另一种动态语法

问题说大不大说小不小，但如果考虑到动态配置的需求，那么toml远远无法满足需求。当前流行的动态配置语言有以下几个：

**KCL**

由CNCF托管的动态配置语言。官网示例：

```kcl
apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name = "nginx"
    labels.app = name
}
spec = {
    replicas = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels = metadata.labels
    template.spec.containers = [
        {
            name = metadata.name
            image = "${metadata.name}:1.14.2"
            ports = [{ containerPort = 80 }]
        }
    ]
}
```

**PKL**

由苹果公司研发的动态配置语言。官方示例：

```pkl
name = "Swallow"

job {
  title = "Sr. Nest Maker"
  company = "Nests R Us"
  yearsOfExperience = 2
}
```

作为大公司或顶级组织管理的项目，流行度和稳定性没的说，对于我来说，存在的最大问题是，难看。设计的都很近似json的组织形式，不如toml优雅。于是我决定自己开发一个基于toml的动态配置语言。这就是faml的由来。
