'use strict';

var path = require('path');
var gulp = require('gulp');
var sass = require('gulp-sass');
var autoprefixer = require('gulp-autoprefixer');
var cleanCSS = require('gulp-clean-css');
var rename = require('gulp-rename');
var sourcemaps = require('gulp-sourcemaps');


var sass_paths = [
    './assets/scss/*.scss',
    './assets/scss/pages/*.scss',
];

gulp.task('sass', function() {
    return gulp.src(sass_paths, { base: './assets/scss'})
        .pipe(sass().on('error', sass.logError))
        .pipe(autoprefixer('last 2 version'))
        .pipe(cleanCSS())
        .pipe(rename(function(path) {
            // path.dirname = path.dirname.replace('/sass', '/css');
            path.extname = '.min.css';
        }))
        .pipe(gulp.dest('./../static'));
});

gulp.task('sass:dev', function () {
    return gulp.src(sass_paths, { base: './assets/scss'})
        .pipe(sourcemaps.init())
        .pipe(sass({
            outputStyle: 'compressed',
            sourceMap: true,
            sourceMapRoot: '/dev',
            includePaths: ['node_modules'],
        }).on('error', sass.logError))
        .pipe(sourcemaps.mapSources(function(sourcePath, _file) {
            return  path.normalize('client/assets/scss/' + sourcePath);
        }))
        .pipe(sourcemaps.write('./../static', {
            sourceMappingURL: function(file) {
                return path.basename(file.path) + '.map';
            }
        }))
        .pipe(gulp.dest('./../static'));
});

gulp.task('sass:watch', function () {
    gulp.watch('./assets/scss/**/*.scss', ['sass:dev']);
});

gulp.task('default', [ 'sass:watch' ]);