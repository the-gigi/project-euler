Profiling
=========
Go to the trivial or islands packages dir and type:

`go test . -bench -cpuprofile prof.cpu`

You should see:

```
> go test . -bench . -cpuprofile prof.cpu
PASS
ok  	_/Users/gigi.sayfan/go/src/github.com/the-gigi/project-euler/8/go/islands	0.007s
```

It will generate a file called prof.cpu

Then use Go's pprof tool to analyze:

```
> go tool pprof prof.cpu

```
