import subprocess
import multiprocessing
import sys

CASE = int(sys.argv[1])
TL = 100

def execute_case(seed):
    input_file_path = f'tools/in/{seed:04}.txt'
    output_file_path = f'tools/out/{seed:04}.txt'
    pipefile = f'tools/memo/pipefile_{seed:04}'
    with open(input_file_path) as fin:
        with open(output_file_path, 'w') as fout:
            with open(pipefile, 'w') as p:
                subprocess.run(['tools/target/release/tester', 'main/target/release/main'], stdin=fin, stdout=fout, stderr=p,timeout=TL)
            output = open(pipefile).read()
    return seed, output

def get_input_data(seed):
    input_file_path = f'tools/in/{seed:04}.txt'
    input_file = open(input_file_path).read()
    N, M = [int(x) for x in input_file.split()[0 : 2]]
    return N, M

def progress(count):
    sys.stdout.write("\033[2K\033[G")
    print(f'{count}/{CASE}', end='', flush=True)


def main():

    scores = []
    count = 1
    scores_dict = {}
    with multiprocessing.Pool(max(1, multiprocessing.cpu_count()-2)) as pool:
        for seed, score in pool.imap_unordered(execute_case, range(CASE)):
            progress(count)
            try:
                score = int(score.split()[2])
                N, _ = get_input_data(seed)
                score /= N * N * 1000000 # ひらいたマス / 全マス
                scores.append((score, f'{seed:04}'))

            except ValueError:
                print()
                print(seed, "ValueError", flush=True)
                print(score, flush=True)
                exit()
            except IndexError:
                print()
                print(seed, "IndexError", flush=True)
                print(f"error: {score}", flush=True)
                exit()
            count += 1
    print()
    scores.sort()
    total = sum([s[0] for s in scores])
    ave = total / CASE
    print(f'total: {total}')
    print(f'max: {scores[-1]}')
    print(f'ave: {ave}')
    print(f'min: {scores[0]}')

    print("==========================")
    for i in range(9):
        print(f'low  {i+1}: {scores[i]}')
    print("==========================")
    for i in range(9):
        print(f'high {i+1}: {scores[-i-1]}')

if __name__ == '__main__':
    main()
