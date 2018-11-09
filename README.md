# cgats

## Read, write, and manipulate [CGATS color files](http://www.colorwiki.com/wiki/CGATS.17_Text_File_Format).

[Rust API Documentation](https://robeirne.github.io/cgats)

### The CGATS format

Here is a basic CGATS color file:

```tsv
CGATS.17
BEGIN_DATA_FORMAT
SAMPLE_ID	SAMPLE_NAME	CMYK_C	CMYK_M	CMYK_Y	CMYK_K
END_DATA_FORMAT
BEGIN_DATA
1	Cyan	100	0	0	0
2	Magenta	0	100	0	0
3	Yellow	0	0	100	0
4	Black	0	0	0	100
5	Blue	100	100	0	0
6	Red	0	100	100	0
7	Green	100	0	100	0
8	3cBlack	100	100	100	0
9	4cBlack	100	100	100	100
10	3cGray	50	40	40	0
11	1cGray	0	0	0	50
END_DATA
```
