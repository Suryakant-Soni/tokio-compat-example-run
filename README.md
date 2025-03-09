# working example for tokio_util::compat compatibility layer

1. a TCP stream object implements tokio::io::AsyncRead
2. as per the compatibility layer the one implementing above interface implements TokioAsyncReadCompatExt and so it gives access to compat layer object using .compat method which passes Compat object back, this object actually now implements futures_io::AsyncReadExt and it will inherit all methods of futures_io trait, 
3. hence calling .read method actually calls futures_io::AsyncReadExt method

How compatibility interchange of trait based methods work - 
because the poll_* methods which is at the heart of async operation in both future and tokio based polling, i.e. which is internally used to read or write data  - is overwritten and implemented as seen in tokio_util::compat, 
for example while implementing poll_read for futures_io::AsyncRead, it call poll_read of tokio::io::AsyncRead internally with some needed tweaks, hence successfully establishing compatibilty between the 2 traits


working of functionality - 
1. we started a tcp stream emitted which is emmitting numbers randomly
2. these are being read using read api of futures trait and printed

