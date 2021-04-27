rm dist/*
trunk build --release
aws s3 cp dist/ s3://lcl-viz --recursive
