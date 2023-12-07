<script>
    import TablePanel from "../../lib/tables/TablePanel.svelte";
    let data=[
        {
            "job_no":"230400341HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400342HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400343HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400344HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400345HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400346HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400347HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400348HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400349HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400350HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400351HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400352HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400353HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400354HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400355HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400356HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400357HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400358HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400359HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400360HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400361HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400362HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400363HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400364HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400365HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400366HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400367HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400368HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400369HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400370HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400366HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400367HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400368HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400369HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400370HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
    ];
    let configs=[
        {header:"Job No",values:["job_no"]},
        {header:"Client",values:["quotation_to"]},
        {header:"Engineers",values:["engineers"]},
        {header:"Services",values:["services"]},
        {header:"Models",values:["models"]},
        {header:"Fee",values:["fee"]}
    ]
</script>


<TablePanel rows={data} columnsConfig={configs}
on:item_db_click={(e)=>console.log(e.detail.row)}
/>