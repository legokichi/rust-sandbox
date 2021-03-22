use tokio_stream::{Stream, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
#[derive(serde::Deserialize, serde::Serialize)]
struct A{}
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let input = tokio::fs::OpenOptions::new()
        .read(true)
        .open("hoge.ndjson")
        .await?;
    let output = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("fuga.ndjson")
        .await?;
    let input = read_ndjson::<A, _>(input).await?;
    // let input = input.filter_map(|json| serde_json::from_value::<A>(json).ok());
    write_ndjson(output, input).await?;
    Ok(())
}

// https://github.com/rust-lang/rust/issues/44721
async fn read_ndjson<T, O>(input: O) -> Result<impl Stream<Item=T>, Box<dyn std::error::Error + Send + Sync>>
where
    T: for<'a> serde::Deserialize<'a>,
    O: AsyncRead,
{
    use tokio_util::codec::{FramedRead, LinesCodec};
    // let input = tokio::io::BufReader::new(input);
    let ndjson = FramedRead::new(input, LinesCodec::new())
        .filter_map(|line| line.ok() )
        .map(|line| serde_json::from_str::<T>(&line) )
        .filter_map(|o| o.ok() );
    Ok(ndjson)
}

async fn write_ndjson(mut output: impl AsyncWrite + Unpin, input: impl Stream<Item=impl serde::Serialize> + Unpin) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use tokio::io::AsyncWriteExt;
    // let output = tokio::io::BufWriter::new(output);
    let mut lines = input
        .filter_map(|json| serde_json::to_string(&json).ok() )
        .map(|line| format!("{}\n", line));
    while let Some(line) = lines.next().await {
        output.write(line.as_bytes()).await?;
    }
    Ok(())
}
