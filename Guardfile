# rubocop:disable LineLength
# rubocop:disable AndOr

require 'sys/proctable'

app_root = ENV['APP_ROOT'] or abort 'Error: You must point the APP_ROOT environment variable to your application root.'

def kill_server_process
  Sys::ProcTable.ps.each do |ps|
    next unless ps.comm.casecmp('copper') == 0
    Process.kill('KILL', ps.pid)
  end
end

last_rebuild = Time.at(0)
debounce = 2 # seconds
guard :shell, all_on_start: true do
  watch(%r{^src/}) do
    since_last = Time.now - last_rebuild
    if since_last > debounce
      kill_server_process
      system "cargo run #{app_root} &"
      last_rebuild = Time.now
    else
      puts "Skipping rebuild after only #{since_last} seconds"
    end
  end
end

at_exit do
  kill_server_process
end
