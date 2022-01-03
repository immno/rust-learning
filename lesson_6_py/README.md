# 构建
在目录下,创建 virtual env，然后用 maturin develop 构建 python 模块：
```shell
python3 -m venv .env
source .env/bin/activate
pip install maturin ipython
maturin develop
```
# 测试
构建完成后，可以用`ipython`测试
```python
import queryer_py
sql = queryer_py.example_sql()
print(queryer_py.query(sql, 'csv'))
print(queryer_py.query(sql, 'json'))
```