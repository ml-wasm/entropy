### To Do
<!-- 
Selection
Use these commands to select a specific subset of your data.

df[col] | Returns column with label col as Series
df[[col1, col2]] | Returns columns as a new DataFrame
s.iloc[0] | Selection by position
s.loc['index_one'] | Selection by index
df.iloc[0,:] | First row
df.iloc[0,0] | First element of first column
 -->

<!-- 
Data Cleaning
Use these commands to perform a variety of data cleaning tasks.

df.columns = ['a','b','c'] | Rename columns
pd.isnull() | Checks for null Values, Returns Boolean Arrray
pd.notnull() | Opposite of pd.isnull()
df.dropna() | Drop all rows that contain null values
df.dropna(axis=1) | Drop all columns that contain null values
df.dropna(axis=1,thresh=n) | Drop all rows have have less than n non null values
df.fillna(x) | Replace all null values with x
s.fillna(s.mean()) | Replace all null values with the mean (mean can be replaced with almost any function from the statistics module)
s.astype(float) | Convert the datatype of the series to float
s.replace(1,'one') | Replace all values equal to 1 with 'one'
s.replace([1,3],['one','three']) | Replace all 1 with 'one' and 3 with 'three'
df.rename(columns=lambda x: x + 1) | Mass renaming of columns
df.rename(columns={'old_name': 'new_ name'}) | Selective renaming
df.set_index('column_one') | Change the index
df.rename(index=lambda x: x + 1) | Mass renaming of index
 -->

<!-- 
Statistics
Use these commands to perform various statistical tests. (These can all be applied to a series as well.)


df.describe() | Summary statistics for numerical columns
df.mean() | Returns the mean of all columns
df.corr() | Returns the correlation between columns in a DataFrame
df.count() | Returns the number of non-null values in each DataFrame column
df.max() | Returns the highest value in each column
df.min() | Returns the lowest value in each column
df.median() | Returns the median of each column
df.std() | Returns the standard deviation of each column
  -->

<!-- - [ ] fn -> desc -->
 - [x] size -> size of df
 - [x] showcolumns -> display col names
 - [ ] renamecolumns -> change column names
 - [x] dtypes -> Datatype of series
 - [ ] iloc -> lookup series with slicing
 - [x] loc