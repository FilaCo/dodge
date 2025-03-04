project('dgc', 'c',
  version : '0.1',
  default_options : ['warning_level=3', 'b_sanitize=address'])

thread_dep = dependency('threads')

inc_dir = include_directories('include')
src = [
  'src/main.c', 
  'src/dgc.c'
]

executable('dgc', 
  sources : src,
  include_directories : inc_dir,
  install : true,
  dependencies : thread_dep)

subdir('test')