const path = require('path');

module.exports = {
    entry: './src/index.js',
    output: {
          path: path.resolve(__dirname, 'public'),
          filename: 'bundle.js',
        },
    mode: 'development',
    devServer: {
        contentBase: path.join(__dirname, 'public')
    }
};
