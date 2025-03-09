# working example for tokio_util::compat compatibility layer

1. a TCP stream object implements tokio::io::AsyncRead
2. as per the compatibility layer the one implementing above interface implements TokioAsyncReadCompatExt and so it gives access to compat layer object using .compat method which passes Compat object back, this object actually now implements futures_io::AsyncReadExt and it will inherit all methods of futures_io trait, 
3. hence calling .read method actually calls futures_io::AsyncReadExt method


working of functionality - 
1. we started a tcp stream emitted which is emmitting numbers randomly
2. these are being read using read api of futures trait and printed

