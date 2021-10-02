# jvm-rs

## boot cofig

```json-with-comments
"args": [
  "-x",
  "./resources",//stimulate class path(too much jar in real classpath, but most time we only need rt.jar)
  "--class",
  "PrintArgs",//the class file you want run
  "--args", // the args be pass to program
  "hello",
  "world"
],
```

## 类加载分析

以下面代码为例简单分析一部分的类加载流程

```java
public class GetClassTest {
    public static void main(String[] args) {
        System.out.println(void.class.getName()); // void
    }
}
```

首先加载 java/lang/Void.class 的时候，在执行 java/lang/Void.class 的 clinit 方法时会加载一个 java string "void" 用于调用 java/lang/Class 的本地方法 getPrimitiveClass，最后给静态变量的 TYPE 赋值

```java
Compiled from "Void.java"
public final class java.lang.Void {
  public static final java.lang.Class<java.lang.Void> TYPE;

  static {};
    Code:
       0: ldc           #1                  // String void
       2: invokestatic  #25                 // Method java/lang/Class.getPrimitiveClass:(Ljava/lang/String;)Ljava/lang/Class;
       5: putstatic     #24                 // Field TYPE:Ljava/lang/Class;
       8: return
    LineNumberTable:
      line 44: 0
}
```

又由于 java String 底层是 char 数组，这个 clinit 方法会进一步引发 字符数组类 `[C` 的加载，因为 char 是基础类型，到这就完了。

但是注意，如果此时调用了 java/lang/Class 中的方法，这个类还没有初始化，导致会执行 java/lang/Class 的 clinit 方法

```java
Compiled from "Class.java"
public final class java.lang.Class<T> implements java.io.Serializable, java.lang.reflect.GenericDeclaration, java.lang.reflect.Type, java.lang.reflect.AnnotatedElement {
static {};
    Code:
       0: invokestatic  #959                // Method registerNatives:()V
       3: iconst_1
       4: putstatic     #919                // Field useCaches:Z
       7: iconst_0
       8: anewarray     #439                // class java/io/ObjectStreamField
      11: putstatic     #920                // Field serialPersistentFields:[Ljava/io/ObjectStreamField;
      14: iconst_0
      15: putstatic     #918                // Field initted:Z
      18: return
    LineNumberTable:
      line 129: 0
      line 2461: 3
      line 3179: 7
      line 3254: 14
}
```

前面的 registerNatives 方法是一个本地方法，这个函数会注册本地方法，但本地方法的注册我已经在 jvm 启动的时候就手动注册好了，所以 registerNatives 对应的实现是一个空方法，相当于啥都没做。

## Unimplemented

没有实现字符串池，导致每个字符串都不共用object。两个字符串比较的时候可能其字面值是相等的，但其底层指向的 object 不相同，导致其结果是 false。

没有实现类文件里用来实现 invokedynamic 指令 的常量类型如 CONSTANT_METHOD_HANDLE。

因为没有实现上面的常量类型，应该是 Java 版本的问题导致后面实现的包括一些 native 方法、变量自动拆装箱、clone 方法、throw 异常等东西无法进行测试。
