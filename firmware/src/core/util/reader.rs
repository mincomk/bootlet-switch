use crate::hal::Serial;

pub async fn read_until(
    serial: &mut impl Serial,
    delimiter: u8,
    buf: &mut [u8],
) -> Result<usize, crate::Error> {
    let mut total_read = 0;

    while total_read < buf.len() {
        let n = serial.read(&mut buf[total_read..]).await?;
        let last_read_range = &buf[total_read..total_read + n];

        defmt::debug!("Read {} bytes", n);

        if let Some(pos) = last_read_range.iter().position(|&b| b == delimiter) {
            return Ok(total_read + pos + 1);
        }

        total_read += n;
    }
    Ok(total_read)
}

pub async fn read_until_cobs(
    serial: &mut impl Serial,
    buf: &mut [u8],
) -> Result<usize, crate::Error> {
    read_until(serial, 0, buf).await
}
