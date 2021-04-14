rm dist/*
yarn run parcel build index.html
rm dist/*.toml
rm dist/*.map
aws s3 cp dist/ s3://lcl-viz --recursive
