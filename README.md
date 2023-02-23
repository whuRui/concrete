# 骨料随机替换算法

该算法可以识别tif文件格式的混凝土三维模型，并将其中的骨料按比例替换成陶粒。

## 安装

要运行这个算法，需要先安装以下依赖：

- numpy
- tifffile
- maturin

```bash
pip install numpy tifffile maturin
```

## 使用


```bash
# build release rust code.
maturin build --release

# install rust pkg which was built from above.
pip install --force-reinstall concrete-0.1.0-cp311-cp311-macosx_10_7_x86_64.whl

python main.py

```

处理后的图片将被保存为 `your-file-name.outs.tif`。

## 参数

你可以在 `main.py` 中调整以下参数：

- `input_file`：输入文件的路径
- `ceramsite_ratio`：陶粒占比

## 示例

以下是一个示例输出：
```
骨料个数: 10 骨料总体积: 100 转换陶粒体积: 70 转换率: 70.0%
calcu duration: 0.123456
```
