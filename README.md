# OpenAIS Architecture Demo application

Project consists of reseach article about building automation system architectures with focus on OpenAIS. Demo application shows communitaion between switches and lights in networked multi-instance GUI application.
Program is written in Rust language with FLTK framework and components communicate with CoAP protocol.

## Run

- Launch controller at port 10000
```bash
./controller/run1.sh
```

- Launch three lights at ports 5000, 5001, 5002
```bash
./light-point/run1.sh
./light-point/run2.sh
./light-point/run3.sh
```

- Launch light switch at ports 4000
```bash
./push-button/run1.sh
```

