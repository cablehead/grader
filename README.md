# grader

<p>
This CLI tool is designed to effectively perform a binary sort of large text
files by categorizing lines into two bins based on user-defined criteria. It
operates by streaming lines to a child process (such as `grep`) and then sorts
these lines based on their echo response from the child process. Lines echoed
back are placed into 'bin1', ideally configured for the most expected case,
while lines not echoed back are categorized into 'bin2'. <img alt="A 1970s cartoon style illustration of a cute robot sorting potatoes"
  src="https://github.com/cablehead/grader/assets/1394/d35f6286-f6b9-4c87-b014-10948f12c8f8"
  width="200">
</p>


This sorting mechanism relies on waiting to see an echoed line before assuming
any omitted lines belong to 'bin2', making it important to configure 'bin1' for
the more frequent case to avoid buffering. The tool is particularly useful for
tasks like parsing log files or any large dataset where binary categorization
is helpful for organization and analysis.

## Example


```
$ cat http.log
192.168.1.1 - - [16/Dec/2023:10:31:45 -0500] "GET /index.html HTTP/1.1" 200 4523
192.168.1.2 - - [16/Dec/2023:10:32:10 -0500] "GET /about.html HTTP/1.1" 200 3498
192.168.1.3 - - [16/Dec/2023:10:33:30 -0500] "POST /login HTTP/1.1" 500 1287 **(Error)**
192.168.1.4 - - [16/Dec/2023:10:34:22 -0500] "GET /contact.html HTTP/1.1" 200 2310
192.168.1.5 - - [16/Dec/2023:10:35:14 -0500] "GET /products.html HTTP/1.1" 200 4981
192.168.1.6 - - [16/Dec/2023:10:36:03 -0500] "GET / HTTP/1.1" 404 1748 **(Error)**
192.168.1.7 - - [16/Dec/2023:10:37:45 -0500] "GET /blog.html HTTP/1.1" 200 3250
192.168.1.8 - - [16/Dec/2023:10:38:52 -0500] "GET /news.html HTTP/1.1" 200 2891
192.168.1.9 - - [16/Dec/2023:10:39:17 -0500] "POST /api/data HTTP/1.1" 500 902 **(Error)**
192.168.1.10 - - [16/Dec/2023:10:40:05 -0500] "GET /terms.html HTTP/1.1" 200 4076
```

```sh

cat http.log | grader ok.log err.log -- grep -v -E "HTTP/1.1\" (500|404)"

```

```
$ cat ok.log
192.168.1.1 - - [16/Dec/2023:10:31:45 -0500] "GET /index.html HTTP/1.1" 200 4523
192.168.1.2 - - [16/Dec/2023:10:32:10 -0500] "GET /about.html HTTP/1.1" 200 3498
192.168.1.4 - - [16/Dec/2023:10:34:22 -0500] "GET /contact.html HTTP/1.1" 200 2310
192.168.1.5 - - [16/Dec/2023:10:35:14 -0500] "GET /products.html HTTP/1.1" 200 4981
192.168.1.7 - - [16/Dec/2023:10:37:45 -0500] "GET /blog.html HTTP/1.1" 200 3250
192.168.1.8 - - [16/Dec/2023:10:38:52 -0500] "GET /news.html HTTP/1.1" 200 2891
192.168.1.10 - - [16/Dec/2023:10:40:05 -0500] "GET /terms.html HTTP/1.1" 200 4076

$ cat err.log
192.168.1.3 - - [16/Dec/2023:10:33:30 -0500] "POST /login HTTP/1.1" 500 1287 **(Error)**
192.168.1.6 - - [16/Dec/2023:10:36:03 -0500] "GET / HTTP/1.1" 404 1748 **(Error)**
192.168.1.9 - - [16/Dec/2023:10:39:17 -0500] "POST /api/data HTTP/1.1" 500 902 **(Error)**
```
