class Transcriptor {
    toRna(dna: string): string {
        return [...dna].reduce((rna: string, nucleotide: string): string => {
            switch (nucleotide) {
                case "G":
                    return `${rna}C`
                case "C":
                    return `${rna}G`
                case "T":
                    return `${rna}A`
                case "A":
                    return `${rna}U`
                default:
                    throw "Invalid input DNA."
            }
        }, "")
    }
}

export default Transcriptor
