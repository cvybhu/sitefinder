# Sitefinder
Autogenerates cool .com domain names and ensures they are free with a dns lookup  

# Example output:
```
Results:
nodasin.com
desedam.com
tibibon.com
bimebe.com
pepisi.com
sotolod.com
senanot.com
sotenop.com
nidene.com
dosemad.com
banaset.com
bedibi.com
manetat.com
nedesos.com
tibelen.com
pebidet.com
tonidem.com
monape.com
nodaban.com
lamitim.com
bisimed.com
pibine.com
tabenel.com
desoden.com
molatim.com
nitamed.com
sosato.com
tisedem.com
```
# Config
You can configure the program by changing values in `src/main.rs` at lines `99` and `100`  
`wanted_names_number`: Number of names to find  
`concurrent_dns_lookups`: Number of site names to test at once, putting higher value here will make the program faster but your computer will be spamming the network more which might anger your DNS provider

# Running
* Install Rust
* Download this repository
* Run `cargo run --release` in this folder
