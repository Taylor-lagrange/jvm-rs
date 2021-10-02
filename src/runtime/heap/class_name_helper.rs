use phf::phf_map;

// 在JVM虚拟机中，存储数据类型的名称时，是使用指定的描述符来存储，而不是我们习惯的 int，float 等。

// Java类型 类型描述符

// int             I
// long            J
// byte            B
// short           S
// char            C
// float           F
// double          D
// boolean         Z
// void            V
// 其他引用类型     L+类全名+;
// 数组            [
// 方法            (参数)返回值

// 3. 示例
// 3.1 例如我们要表示一个 String 类
//
// Java 类型：java.lang.String
// JNI 描述符：Ljava/lang/String;
// 即一个 Java 类对应的描述符，就是 L 加上类的全名，其中 . 要换成 / ，最后 不要忘掉末尾的分号。
//
// 3.2 假如我们想要表示数组的话
//
// Java 类型：String[]
// JNI 描述符：[Ljava/lang/String;
// Java 类型：int[][]
// JNI 描述符：[[I
// 数组就是简单的在类型描述符前加 [ 即可，二维数组就是两个 [ ，以此类推。
//
// 3.3 方法
//
// Java 方法：long f (int n, String s, int[] arr);
// JNI 描述符：(ILjava/lang/String;[I)J
// Java 方法：void f ();
// JNI 描述符：()V
// 括号内是每个参数的类型符，括号外就是返回值的类型符。

pub static PRIMITIVE_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
  "void"=>   "V",
  "boolean"=> "Z",
  "byte"=>   "B",
  "short"=>   "S",
  "int"=>    "I",
  "long"=>    "J",
  "char"=>    "C",
  "float"=>   "F",
  "double"=>  "D",
};

// [XXX -> [[XXX
// int -> [I
// XXX -> [LXXX;
pub fn get_array_class_name(class_name: String) -> String {
    "[".to_string() + to_descriptor(class_name).as_str()
}

// [[XXX -> [XXX
// [LXXX; -> XXX
// [I -> int
// let ss: String = s.chars().skip(7).take(5).collect();  "Hello, world!" -> "world"
pub fn get_component_class_name(class_name: String) -> String {
    if class_name.starts_with('[') {
        let component_type_descriptor: String = class_name.chars().skip(1).collect();
        return to_class_name(component_type_descriptor);
    }
    panic!("Not array: {}", class_name)
}

// [XXX  => [XXX
// LXXX; => XXX
// I     => int
fn to_class_name(descriptor: String) -> String {
    if descriptor.starts_with('[') {
        //array
        return descriptor;
    }
    if descriptor.starts_with('L') {
        // object
        return descriptor.chars().skip(1).collect();
    }
    for key in PRIMITIVE_TYPES.keys() {
        if *key == descriptor {
            return PRIMITIVE_TYPES[*key].to_string();
        }
    }
    panic!("Invalid descriptor: {}", descriptor);
}

// [XXX => [XXX
// int  => I
// XXX  => LXXX;
fn to_descriptor(mut class_name: String) -> String {
    if class_name.starts_with('[') {
        return class_name;
    }
    if PRIMITIVE_TYPES.contains_key(&class_name) {
        return PRIMITIVE_TYPES[class_name.as_str()].to_string();
    }
    class_name.push(';');
    "L".to_string() + class_name.as_str()
}
