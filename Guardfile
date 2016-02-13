# rubocop:disable LineLength
# rubocop:disable AndOr

require 'sys/proctable'

app_root = ENV['APP_ROOT'] or abort 'Error: You must point the APP_ROOT environment variable to your application root.'

class GFilter
  def self.run(files, &_block)
    @mtimes ||= {}

    files.each do |f|
      mtime = File.mtime(f)
      next if @mtimes[f] == mtime
      @mtimes[f] = mtime

      yield f
    end
  end
end

def kill_server_process
  Sys::ProcTable.ps.each do |ps|
    next unless ps.comm.casecmp('copper') == 0
    Process.kill('KILL', ps.pid)
  end
end

guard :shell, all_on_start: true do
  watch(%r{^src/}) do |m|
    # Tends to get called multiple times on startup, hence this is needed.
    GFilter.run(m) do
      kill_server_process
      system "cargo run #{app_root} &"
    end
  end
end

at_exit do
  kill_server_process
end
